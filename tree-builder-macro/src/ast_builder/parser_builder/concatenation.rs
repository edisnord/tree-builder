use crate::parser::parser_ast::{ConcatKind, Concatenation, Include, Factor, Term};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use crate::ast_builder::parser_builder::analyze_groupings;

pub fn concatenation(ref mut conc: &mut Concatenation) -> (TokenStream2, TokenStream2) {
    let mut all: Vec<TokenStream2> = vec![];
    let mut names: Vec<TokenStream2> = vec![];
    let closures: TokenStream2 = analyze_groupings(conc);
    let mut number: usize = 0;
    for conc in &mut(conc.0) {
        all.push(match conc {
            ConcatKind::Include(Include(fact)) => {
                let name = format_ident!("res{}", number);
                let post =
                    if matches!(fact, Factor::Optional(Term::Terminal(_) | Term::Metacharacter(_))) {
                        quote!{
                            let #name: std::option::Option<std::string::String> = #name.map(|x|x.to_owned());
                        }
                    } else if matches!(fact, Factor::ZeroOrMore(Term::Terminal(_)) | Factor::OneOrMore(Term::Terminal(_))) {
                        quote!{
                            let #name: std::vec::Vec<std::string::String> = #name.into_iter().map(|x|x.to_owned()).collect();
                        }
                    } else if fact.is_metachar() || fact.is_terminal() {
                        quote!{let #name: String = #name.to_owned();}
                    }
                    else {
                        quote!{}
                    };
                let fact = super::factor::factor(fact);
                let name = format_ident!("res{}", number);
                names.push(quote!(#name));
                number += 1;
                quote! {
                    let (input, #name) = #fact (input)?;
                    #post
                }
            }
            ConcatKind::Factor(fact) => {
                let fact = super::factor::factor(fact);
                quote! {
                    let (input, _) = #fact (input)?;
                }
            }
        });
    }
    (
        quote!(#closures
            #(#all)
           *),
        quote!(#(#names), *),
    )
}
