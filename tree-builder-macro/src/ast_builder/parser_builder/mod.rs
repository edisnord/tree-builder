mod concatenation;
mod factor;

use crate::{
    parser::parser_ast::{Alternation, ConcatKind, Concatenation, Factor, Grouping, Include, Term},
    regex_builder::term::{identifier, metacharacter, terminal}, ast_builder::IncludeAnalysis,
};
use std::any::Any;

fn term(term: &Term) -> TokenStream2 {
    match term {
        Term::Metacharacter(meta) => metacharacter(meta),
        Term::Ident(ident) => identifier(ident),
        Term::Terminal(term) => terminal(term),
        Term::Grouping(Grouping(_, ref name)) => {
            let ident = format_ident!("{}", name.as_ref().unwrap());
            quote! {#ident}
        }
    }
}

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};

use crate::parser::parser_ast::{Rhs, StructRule};
fn alternation(
    (num,
    alt): (usize, &mut Alternation),
) -> TokenStream2 {
    let (body, names) = concatenation::concatenation(&mut alt.concatenation);
    let name = get_variant_name(alt).unwrap();
    let fun_name = format_ident!("alt{}", num);
    quote! {
        let #fun_name = |input: &'a str| {
                        #body
                        Ok((input, Self:: #name (#names)))
                    };
    }
}

fn righths(Rhs(alts): &mut Rhs) -> TokenStream2 {
    if alts.len() == 1 {
        let Alternation {
            ref mut concatenation,
            ..
        } = alts[0];
        let (parsed, names) = concatenation::concatenation(concatenation);
        quote! {
            #parsed
            let result = Self(#names);
        }
    } else {
        let closures: Vec<TokenStream2> = alts.iter_mut().enumerate().map(alternation).collect();
        let names: Vec<TokenStream2> = alts
            .iter().enumerate()
            .map(|(num, _)| {
                let ident = format_ident!("alt{}", num);
                quote! {#ident}
            })
            .collect();
        let closures = quote! {
            #(#closures)
                *
        };
        quote! {
            #closures
            let (input, result) = tree_builder::__private::nom::branch::alt((#(#names), *))(input)?;
        }
    }
}

pub fn gen_parser(
    StructRule { lhs, ref mut rhs }: &mut StructRule,
    _derived_type: Option<&dyn Any>,
) -> TokenStream2 {
    let rhs = righths(rhs);
    let ident = format_ident!("{}", lhs);
    quote! {
        impl tree_builder::Parser for #ident {
            fn parse<'a>(input: &'a str) -> tree_builder::__private::nom::IResult<&str, std::boxed::Box<#ident>, tree_builder::__private::nom::error::VerboseError<&str>> {
                #rhs
                return Ok((input, std::boxed::Box::new(result)))
            }
        }
    }
}

//fn derived_parser(_input: &StructRule) -> TokenStream2 {
//    todo!()
//}

pub fn analyze_groupings<'a>(Concatenation(concs): &'a mut Concatenation) -> TokenStream2 {
    let mut closures: Vec<TokenStream2> = vec![];
    let mut count: usize = 0usize;
    for kind in concs {
        let factor = match kind {
            ConcatKind::Factor(factor) => factor,
            ConcatKind::Include(Include(factor)) => factor,
        };
        let def_ret_val = quote!{()};
        let term = match factor {
            Factor::Term(ref mut term) => term,
            Factor::Optional(ref mut term) => term,
            Factor::OneOrMore(ref mut term) => term,
            Factor::ZeroOrMore(ref mut term) => term,
        };
        let temp_term = term.clone();
        if let Term::Grouping(ref mut group @ Grouping(_, _)) = term {
            group.1 = Some(format!("group{}", count));
            count = count + 1usize;
            let ret_type = {
                let temp_factor = Factor::Term(temp_term);
                let analysis = IncludeAnalysis::new(&temp_factor);
                analysis.generate_type()
            };
            let (rhs, mut names) = concatenation::concatenation(group.0.as_mut());
            if names.is_empty() {
                names = def_ret_val;
            }
            let ident = format_ident!("{}", group.1.as_ref().unwrap());
            let closure = quote! {
                let #ident: fn(&'a str) -> tree_builder::__private::nom::IResult<&str, #ret_type , tree_builder::__private::nom::error::VerboseError<&str>> =
                |input: &'a str| {
                        #rhs
                        Ok((input, (#names)))
                };
            };
            closures.push(closure);
        };
    }
    quote! {#(#closures)
    *}
}

pub fn get_variant_name(
    Alternation {
        concatenation,
        identifier,
    }: &Alternation,
) -> Option<TokenStream2> {
    let Concatenation(factors) = concatenation;
    if identifier.is_none() {
        if let &[ConcatKind::Include(Include(ref factor))] = factors.as_slice() {
            match factor {
                Factor::Term(Term::Ident(ident))
                | Factor::Optional(Term::Ident(ident))
                | Factor::OneOrMore(Term::Ident(ident))
                | Factor::ZeroOrMore(Term::Ident(ident)) => {
                    let ident = format_ident!("{}", ident);
                    Some(quote!(#ident))
                }
                _ => None,
            }
        } else {
            None
        }
    } else {
        let ident = format_ident!("{}", identifier.as_ref().unwrap());
        Some(quote!(#ident))
    }
}
