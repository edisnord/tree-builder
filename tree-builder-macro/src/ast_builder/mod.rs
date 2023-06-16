use proc_macro2::TokenStream as TokenStream2;
pub mod parser_builder;
mod tuple_type_builder;
pub use tuple_type_builder::{ast_from_rule, ast_from_rule_no_alts};

use crate::parser::parser_ast::{
    ConcatKind, Concatenation, Factor, Grouping, Include, Term,
};
use quote::quote;

#[derive(Debug)]
struct IncludeAnalysis<'a> {
    children: Vec<IncludeAnalysis<'a>>,
    node: &'a Factor,
}

impl<'a> IncludeAnalysis<'a> {
    /// Construct an IncludeAnalysis object out of a factor consisting of a reference to
    /// the factor itself, and a vector of other IncludeAnalysis objects, which will be
    /// other factors which have been included inside the input if it is a grouping,
    /// or an empty vec if it is another kind of include
    pub fn new(factor: &'a Factor) -> Self {
        let children: Vec<Self> = match factor {
            Factor::Term(Term::Grouping(Grouping(conc, _)))
            | Factor::Optional(Term::Grouping(Grouping(conc, _)))
            | Factor::OneOrMore(Term::Grouping(Grouping(conc, _)))
            | Factor::ZeroOrMore(Term::Grouping(Grouping(conc, _))) => {
                Self::analyze_concatenation(conc)
            }
            _ => vec![],
        };
        IncludeAnalysis {
            children,
            node: factor,
        }
    }

    pub fn analyze_concatenation(conc: &'a Concatenation) -> Vec<Self> {
        let mut result = vec![];
        for next in &(conc.0) {
            match next {
                ConcatKind::Factor(_) => (),
                ConcatKind::Include(Include(next)) => result.push(Self::new(next)),
            }
        }
        result
    }

    pub fn generate_type(&self) -> TokenStream2 {
        let Self { children: _, node } = self;
        let (wrapper, term): (fn(TokenStream2) -> TokenStream2, &Term) = match node {
            Factor::Optional(term) => (|x: TokenStream2| quote! {std::option::Option<#x>}, term),
            Factor::OneOrMore(term) => (|x: TokenStream2| quote! {std::vec::Vec<#x>}, term),
            Factor::ZeroOrMore(term) => (|x: TokenStream2| quote! {std::vec::Vec<#x>}, term),
            Factor::Term(term) => (|x: TokenStream2| quote! {#x}, term),
        };
        if !node.is_grouping() {
            wrapper(Self::term_to_type(term))
        } else {
            wrapper(Self::type_from_grouping(self))
        }
    }

    fn term_to_type(term: &Term) -> TokenStream2 {
        match term {
            Term::Metacharacter(_) | Term::Terminal(_) => quote!(String),
            Term::Ident(ident) => {
                let ident = proc_macro2::Ident::new(ident, proc_macro2::Span::call_site());
                quote!(std::boxed::Box<#ident>)
            }
            _ => panic!("Do not call with grouping"),
        }
    }

    fn type_from_grouping(analysis: &Self) -> TokenStream2 {
        let wrapper = if analysis.children.len() > 1 {
            |x: Vec<TokenStream2>| quote!((#(#x),*))
        } else {
            |x: Vec<TokenStream2>| {
                let x = &x[0];
                quote!(#x)
            }
        };
        wrapper(analysis.children.iter().map(Self::generate_type).collect())
    }

    pub fn concat_to_type(concatenation: &Concatenation) -> TokenStream2 {
        let analysis = IncludeAnalysis::analyze_concatenation(concatenation);
        let analysis: Vec<TokenStream2> = analysis
            .iter()
            .map(IncludeAnalysis::generate_type)
            .collect();
        quote!((#(#analysis),*))
    }


}


