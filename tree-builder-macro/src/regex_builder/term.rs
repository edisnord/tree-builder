use crate::parser::parser_ast::{Metacharacter, RegGrouping, RegTerm, Terminal};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::LitStr;

pub fn terminal(Terminal(term): &Terminal) -> TokenStream2 {
    let qualifier = quote! {tree_builder::__private::nom::bytes::complete::tag};
    let generics = quote! {::<_, _, tree_builder::__private::nom::error::VerboseError<_>>};
    let literal = LitStr::new(term, Span::call_site());
    quote! {#qualifier #generics(#literal)}
}

pub fn identifier(ident: &str) -> TokenStream2 {
    let ident = format_ident!("{}", ident);
    quote! {#ident::parse}
}

pub fn metacharacter(metachar: &Metacharacter) -> TokenStream2 {
    match metachar {
        Metacharacter::Digits => quote!{|x:char| x.is_digit(10)},
        Metacharacter::AllChars => quote!{|x:char| x != '\n'},
        Metacharacter::NonDigits => quote!{|x:char| !x.is_digit(10)},
        Metacharacter::Whitespace => quote!{|x:char| x.is_whitespace()},
        Metacharacter::NonWhitespace => quote!{|x:char| !x.is_whitespace()},
        Metacharacter::AlphaNumericUnderscore => quote!{|x:char| !(x.is_alphanumeric() || x == '_')},
        Metacharacter::NonAlphaNumericUnderscore => quote!{|x:char| !(x.is_alphanumeric() || x == '_')},
        Metacharacter::SquareBrackets(values) => {
            let list: String = values.into_iter().collect();
            let literal = LitStr::new(&list, Span::call_site());
            quote! {|x:char| #literal .contains(x)}
        }
        Metacharacter::ExcludingSquareBrackets(values) => {
            let list: String = values.into_iter().collect();
            let literal = LitStr::new(&list, Span::call_site());
            quote! {|x:char| !#literal .contains(x)}
        }
    }
}

pub fn term(term: &RegTerm) -> TokenStream2 {
    match term {
        RegTerm::Metacharacter(meta) => metacharacter(meta),
        RegTerm::Ident(ident) => identifier(ident),
        RegTerm::Terminal(term) => terminal(term),
        RegTerm::Grouping(RegGrouping(_, name)) => match name {
            Some(name) => {
                let name = format_ident!("{}", name);
                quote! {#name}
            }
            None => panic! {"Error detecting grouping"},
        },
    }
}
