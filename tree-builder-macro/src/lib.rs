mod regex_builder;
mod ast_builder;
mod parser;
extern crate proc_macro;

use proc_macro::TokenStream;

trait Parser {
    fn parse(input: &str) -> Self;
}

#[proc_macro]
pub fn build_tree(_: TokenStream) -> TokenStream {
    todo!()
}

#[proc_macro]
pub fn regex(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let input: Vec<&str> = input.split_whitespace().collect();
    let input = input.join("");
    let mut ast = match parser::reg_rule(&input){
        Ok((_, out)) => out,
        Err(nom::Err::Error(e)) => panic!("{}", nom::error::convert_error(input.as_str(), e)),
        _ => panic!("??")
    };
    regex_builder::rule(&mut ast).into()
}

#[proc_macro]
pub fn ast(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let input: Vec<&str> = input.split_whitespace().collect();
    let input = input.join("");
    let ast = match parser::ast_rule(&input){
        Ok((_, out)) => out,
        Err(nom::Err::Error(e)) => panic!("{}", nom::error::convert_error(input.as_str(), e)),
        _ => panic!("??")
    };
    ast_builder::ast_from_rule(&ast).into()
}
