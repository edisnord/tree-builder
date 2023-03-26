use super::parser::specification;
use quote::{quote, __private::TokenStream, format_ident};
use super::parser::parser_ast::*;

// pub fn ast_gen(input: TokenStream) -> TokenStream {
//     let source = input.to_string();
//     let (rest, tree) = specification(&source).unwrap();
//     if !rest.is_empty() {
//         panic!("Source not fully parsed for some reason")
//     }
    
//     for rule in tree.0 {
//         let Rule{lhs, rhs} = rule;
//         let Rhs(alternations) = *rhs;
//         for alternation in alternations {
//             let Alternation { concatenation, identifier } = alternation;
//             let identifier = identifier.unwrap_or(lhs.clone());
//             for factor in concatenation.0 {
//                 match factor {
//                     Factor::Optional(term) => {

//                     },
//                     Factor::ZeroOrMore(term) => {},
//                     Factor::OneOrMore(term) => {},
//                     Factor::Term(term) => {},
//                 }
//             }
//         }
    
//     }



//     let gen = quote!{};

//     gen.into()
// }