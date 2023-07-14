mod regex_builder;
mod ast_builder;
mod parser;
extern crate proc_macro;

use proc_macro::TokenStream;
use parser::parser_ast::RuleKind;
use quote::quote;
use regex::Regex;

use crate::parser::parser_ast::Specification;

trait Parser {
    fn parse(input: &str) -> Self;
}

#[proc_macro]
pub fn build_tree(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let input: Vec<&str> = input.split_whitespace().collect();
    let input = input.join("");
    let Specification(mut ast) = match parser::specification(&input){
        Ok((_, out)) => out,
        Err(nom::Err::Error(e) | nom::Err::Failure(e)) => panic!("{}", nom::error::convert_error(input.as_str(), e)),
        _ => panic!("Unknown error. Please report this error on GitHub issues, together with the grammar you used")
    };
    let tokens: Vec<proc_macro2::TokenStream> = ast.iter_mut().map(rule_kind_to_stream).collect();
    quote!{
        #(#tokens)
            *
    }.into()
}

#[proc_macro]
pub fn ast_parser_maker(input: TokenStream) -> TokenStream {
    let comment = Regex::new(r"//.*$").unwrap();
    let input = input.to_string();
    let input: Vec<String> = input.split_whitespace().map(|x| comment.replace_all(x, "").to_string()).collect();
    let input = input.join("");
    let mut ast = match parser::struct_rule(&input){
        Ok((_, out)) => out,
        Err(nom::Err::Error(e) | nom::Err::Failure(e)) => panic!("{}", nom::error::convert_error(input.as_str(), e)),
        _ => panic!("Unknown error. Please report this error on GitHub issues, together with the grammar you used")
    };
    ast_builder::parser_builder::gen_parser(&mut ast, None).into()
}


#[proc_macro]
pub fn rule(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let input: Vec<&str> = input.split_whitespace().collect();
    let input = input.join("");
    let mut ast = match parser::rule(&input){
        Ok((_, out)) => out,
        Err(nom::Err::Error(e)) => panic!("{}", nom::error::convert_error(input.as_str(), e)),
        _ => panic!("Unknown error, please open a github issue which includes your source code")
    };
    rule_kind_to_stream(&mut ast).into()
}

fn rule_kind_to_stream(ast: &mut RuleKind) -> proc_macro2::TokenStream {
    match ast {
        RuleKind::StructRule(ref mut ast) => {
            let tuple_type = ast_builder::ast_from_rule(ast);
            let parser = ast_builder::parser_builder::gen_parser(ast, None);
            quote!{
                #tuple_type

                #parser
            }.into()
        },
        RuleKind::RegexRule(ref mut ast) => {
            regex_builder::rule(ast).into()
        }
    }
}
