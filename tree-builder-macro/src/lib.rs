mod regex_builder;
mod ast_builder;
mod parser;
extern crate proc_macro;

use proc_macro::TokenStream;
use parser::parser_ast::RuleKind;
use quote::quote;

trait Parser {
    fn parse(input: &str) -> Self;
}

#[proc_macro]
pub fn build_tree(_: TokenStream) -> TokenStream {
    todo!()
}

#[proc_macro]
pub fn ast_parser_maker(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let input: Vec<&str> = input.split_whitespace().collect();
    let input = input.join("");
    let mut ast = match parser::struct_rule(&input){
        Ok((_, out)) => out,
        Err(nom::Err::Error(e)) => panic!("{}", nom::error::convert_error(input.as_str(), e)),
        _ => panic!("??")
    };
    ast_builder::parser_builder::gen_parser(&mut ast, None).into()
}


#[proc_macro]
pub fn rule(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let input: Vec<&str> = input.split_whitespace().collect();
    let input = input.join("");
    let ast = match parser::rule(&input){
        Ok((_, out)) => out,
        Err(nom::Err::Error(e)) => panic!("{}", nom::error::convert_error(input.as_str(), e)),
        _ => panic!("??")
    };
    match ast {
        RuleKind::StructRule(mut ast) => {
            let tuple_type = ast_builder::ast_from_rule(&ast);
            let parser = ast_builder::parser_builder::gen_parser(&mut ast, None);
            quote!{
                #tuple_type

                #parser
            }.into()
        },
        RuleKind::RegexRule(mut ast) => {
            regex_builder::rule(&mut ast).into()
        }
    }
}
