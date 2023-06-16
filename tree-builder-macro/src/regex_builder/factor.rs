use crate::parser::parser_ast::RegTerm;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};

pub fn option_invocation(term: &RegTerm) -> TokenStream2 {
    let term = if matches!(term, RegTerm::Metacharacter(_)) {
        term_invocation(term)
    } else {
        super::term::term(term)
    };
    quote! {tree_builder::__private::nom::combinator::opt(#term)}
}

pub fn option_post(term: &RegTerm) -> TokenStream2 {
    let (decl, unwrap) = match term {
        RegTerm::Ident(ident) => {
            let ident = format_ident!("{}", ident);
            (
                quote! {let #ident (res)},
                quote! {*res.unwrap_or_else(|| Box::new(#ident (String::new())))},
            )
        },
        RegTerm::Metacharacter(_) | RegTerm::Terminal(_) => {
            (
                quote! {let res},
                quote! {res.map(|x|x.to_owned()).unwrap_or_else(|| String::new())}
                )
        }
        _ => (
            quote! {let res},
            quote! {res.unwrap_or_else(||String::new())},
        ),
    };
    quote! {#decl = #unwrap ;}
}

pub fn many1(term: &RegTerm) -> TokenStream2 {
    let term_stream = super::term::term(term);
    if matches!(term, RegTerm::Metacharacter(_)) {
        quote! {tree_builder::__private::nom::bytes::complete::take_while1(#term_stream)}
    } else {
        quote! {tree_builder::__private::nom::multi::many1(#term_stream)}
    }
}

pub fn many0(term: &RegTerm) -> TokenStream2 {
    let term_stream = super::term::term(term);
    if matches!(term, RegTerm::Metacharacter(_)) {
        quote! {tree_builder::__private::nom::bytes::complete::take_while(#term_stream)}
    } else {
        quote! {tree_builder::__private::nom::multi::many0(#term_stream)}
    }
}

pub fn term_invocation(term: &RegTerm) -> TokenStream2 {
    let cond = super::term::term(term);

    match term {
        RegTerm::Metacharacter(_) => {
            quote! {tree_builder::public_parsers::satisfy_one(#cond)}
        }
        _ => cond,
    }
}
