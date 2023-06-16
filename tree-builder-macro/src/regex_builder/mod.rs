use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
mod concatenation;
pub mod term;
mod factor;

use crate::parser::parser_ast::{RegAlternation, RegGrouping, RegRhs, RegRule, RegTerm};

pub fn owned_push() -> TokenStream2 {
    quote! {result.push(res.as_str());}
}

pub fn standard_push() -> TokenStream2 {
    quote! {result.push(res);}
}


// what
pub fn alternation(RegAlternation(conc): &RegAlternation) -> TokenStream2 {
    let concat = concatenation::concatenation(conc);
    quote! {
        #concat
    }
}


// Needs refactoring i think
pub fn righths<'a>(rhs: &'a mut RegRhs) -> TokenStream2 {
    let gr = analyze_groupings(rhs);
    let alts: &mut Vec<RegAlternation> = rhs.0.as_mut();
    let alt = if alts.len() == 1 {
        alternation(alts.get(0).unwrap())
    } else {
        let name_mapper =
            |(i, alt): (usize, &'a RegAlternation)| -> (TokenStream2, &'a RegAlternation) {
                let i = syn::Index::from(i);
                let name = format_ident!("alt{}", i);
                (quote! {#name}, alt)
            };

        let closure_mapper =
            |(name, RegAlternation(conc)): (TokenStream2, &'a RegAlternation)| -> TokenStream2 {
                let body = concatenation::concatenation(conc);
                quote! {
                    let #name = |input| {
                                    let mut result: std::vec::Vec<&str> = std::vec::Vec::new();
                                    #body
                                    let mut result = result.join("");
                                    Ok((input, result))
                                };
                }
            };

        let closures: Vec<TokenStream2> = alts
            .iter()
            .enumerate()
            .map(name_mapper)
            .map(closure_mapper)
            .collect();

        let names: Vec<TokenStream2> = alts.iter().enumerate().map(|a| name_mapper(a).0).collect();
        let push = owned_push();
        quote! {
            #(#closures)
              *

            let (input, res) = tree_builder::__private::nom::branch::alt((#(#names),*))(input)?;
            #push
        }
    };
    quote! {
        let mut result: std::vec::Vec<&str> = std::vec::Vec::new();
        #gr
        #alt
        let mut result = result.join("");
    }
}

pub fn rule(RegRule { lhs, rhs }: &mut RegRule) -> TokenStream2 {
    let rhs = righths(rhs);
    let ident = format_ident!("{}", lhs);
    quote! {
        #[derive(Debug)]
        struct #ident(String);

        impl tree_builder::Parser for #ident {
            fn parse(input: &str) -> tree_builder::__private::nom::IResult<&str, std::boxed::Box<Self>, tree_builder::__private::nom::error::VerboseError<&str>> {
                #rhs
                Ok((input, std::boxed::Box::new(Self(result))))
            }
        }
    }
}

fn analyze_groupings<'a>(RegRhs(alts): &'a mut RegRhs) -> TokenStream2 {
    use crate::parser::parser_ast::RegFactor;
    let mut closures: Vec<TokenStream2> = vec![];
    let mut count: usize = 0usize;
    for alt in alts {
        let conc = &mut (alt.0);
        for factor in &mut (conc.0) {
            let term = match factor {
                RegFactor::Term(ref mut term) => term,
                RegFactor::Optional(ref mut term) => term,
                RegFactor::OneOrMore(ref mut term) => term,
                RegFactor::ZeroOrMore(ref mut term) => term,
            };
            if let RegTerm::Grouping(ref mut group @ RegGrouping(_, _)) = term {
                group.1 = Some(format!("group{}", count));
                count = count + 1usize;
                let rhs = righths(group.0.as_mut());
                let ident = format_ident!("{}", group.1.as_ref().unwrap());
                let closure = quote! {
                    let #ident: fn(&str) -> tree_builder::__private::nom::IResult<&str, String, tree_builder::__private::nom::error::VerboseError<&str>> =
                    |input| {
                            let mut result: std::vec::Vec<&str> = std::vec::Vec::new();
                            #rhs
                            Ok((input, result))
                    };
                };
                closures.push(closure);
            };
        }
    }
    quote! {#(#closures)
    *}
}
