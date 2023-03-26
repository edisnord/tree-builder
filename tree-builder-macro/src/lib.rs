mod parser;
extern crate proc_macro;


use parser::parser_ast::{Alternation, Metacharacter, Rule};
use proc_macro::TokenStream;
use quote::{format_ident, quote};

type TokenStream2 = quote::__private::TokenStream;

#[proc_macro]
pub fn build_tree(input: TokenStream) -> TokenStream {
    panic!("kys");
    todo!()
}

fn ast_gen1() -> TokenStream2 {
    let item1 = format_ident!("{}", "Asdf1");
    quote! {
        #item1(i32),
    }
}

fn ast_gen2() -> TokenStream2 {
    quote! {
        Asdf2(i32),
    }
    .into()
}

// #[proc_macro]
// pub fn ast_gen(input: TokenStream) -> TokenStream {
//     let gen = quote!{};

//     gen.into()
// }

// mod parser;
// mod builder;
