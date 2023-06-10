mod regex_builder;
mod tree_builder;
mod parser;
extern crate proc_macro;

// use public_parsers::digit;
use regex_builder::*;
use proc_macro::TokenStream;

trait Parser {
    fn parse(input: &str) -> Self;
}

#[proc_macro]
pub fn build_tree(input: TokenStream) -> TokenStream {
    todo!()
}

#[proc_macro]
pub fn tb_regex(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let input: Vec<&str> = input.split_whitespace().collect();
    let input = input.join("");
    eprintln!("{}", input);
    let mut ast = match parser::rule(&input){
        Ok((_, out)) => out,
        Err(nom::Err::Error(e)) => panic!("{}", nom::error::convert_error(input.as_str(), e)),
        _ => panic!("??")
    };
    rule(&mut ast).into()
}

#[proc_macro_derive(Parser)]
pub fn tb_parser(input: TokenStream) -> TokenStream {
    todo!()
}
