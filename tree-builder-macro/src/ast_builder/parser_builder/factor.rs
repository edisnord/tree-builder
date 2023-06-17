use crate::parser::parser_ast::{Factor, Term};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

fn option(term: &Term) -> TokenStream2 {
    let term = if matches!(term, Term::Metacharacter(_)) {
        let meta = super::term(term);
        quote! {tree_builder::public_parsers::satisfy_one(#meta)}
    } else {
        super::term(term)
    };
    quote! {tree_builder::__private::nom::combinator::opt(#term)}
}

fn many1(term: &Term) -> TokenStream2 {
    let term_stream = super::term(term);
    if matches!(term, Term::Metacharacter(_)) {
        quote! {tree_builder::__private::nom::bytes::complete::take_while1(#term_stream)}
    } else {
        quote! {tree_builder::__private::nom::multi::many1(#term_stream)}
    }
}

fn many0(term: &Term) -> TokenStream2 {
    let term_stream = super::term(term);
    if matches!(term, Term::Metacharacter(_)) {
        quote! {tree_builder::__private::nom::bytes::complete::take_while(#term_stream)}
    } else {
        quote! {tree_builder::__private::nom::multi::many0(#term_stream)}
    }
}

pub fn factor(fact: &Factor) -> TokenStream2 {
    match fact {
        Factor::Optional(term) => option(term),
        Factor::OneOrMore(term) => many1(term),
        Factor::ZeroOrMore(term) => many0(term),
        Factor::Term(term) => {
            let cond = super::term(term);
            match term {
                Term::Metacharacter(_) => {
                    quote! {tree_builder::public_parsers::satisfy_one(#cond)}
                },
                _ => cond
            }
        }
    }
}
