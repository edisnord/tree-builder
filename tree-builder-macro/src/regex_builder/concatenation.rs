use crate::parser::parser_ast::{RegConcatenation, RegFactor, RegTerm};
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, format_ident};

pub fn concatenation(RegConcatenation(concs): &RegConcatenation) -> TokenStream2 {
    let declaration = quote! {let (input, res) = };
    let mapper = |factor: &RegFactor| match factor {
        RegFactor::Term(term) => {
            let push = if matches!(term, RegTerm::Grouping(_)) {
                super::owned_push()
            } else {
                super::standard_push()
            };
            let invocation = super::factor::term_invocation(term);
            quote! {
                #declaration #invocation(input)?;
                #push
            }
        }
        RegFactor::Optional(term) => {
            let invocation = super::factor::option_invocation(term);
            let post = super::factor::option_post(term);
            let push = super::owned_push();
            quote! {
                #declaration #invocation(input)?;
                #post
                #push
            }
        }
        RegFactor::ZeroOrMore(term) => {
            let invocation = super::factor::many0(term);
            let (push, post) = match term {
                RegTerm::Ident(typ) => {
                    let ident = format_ident!("{}", typ);
                    (super::owned_push(),
                     quote!{
                        let res: std::vec::Vec<std::string::String> = res.into_iter().map(|inp| {let inp = *inp; let #ident (inp) = inp; inp}).collect();
                        let res = res.join("");
                     })},
                RegTerm::Terminal(_) | RegTerm::Grouping(_)=> {
                    (super::owned_push(), quote!{let res = res.join("");})
                }
                _ => (super::standard_push(), quote!{})
            };
            quote! {
                #declaration #invocation(input)?;
                #post
                #push
            }
        }
        RegFactor::OneOrMore(term) => {
            let invocation = super::factor::many1(term);
            let (push, post) = match term {
                RegTerm::Ident(typ) => {
                    let ident = format_ident!("{}", typ);
                    (super::owned_push(),
                     quote!{
                        let res: std::vec::Vec<std::string::String> = res.into_iter().map(|inp| {let inp = *inp; let #ident (inp) = inp; inp}).collect();
                        let res = res.join("");
                     })},
                RegTerm::Terminal(_) | RegTerm::Grouping(_)=> {
                    (super::owned_push(), quote!{let res = res.join("");})
                }
                _ => (super::standard_push(), quote!{})
            };
            quote! {
                #declaration #invocation(input)?;
                #post
                #push
            }
        }
    };
    let concs: Vec<TokenStream2> = concs.into_iter().map(mapper).collect();
    quote! {
        #(#concs)
        *
    }
}
