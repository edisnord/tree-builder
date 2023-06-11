use crate::parser::parser_ast::{Alternation, Rhs};
use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::{Ident, Span};

use crate::parser::parser_ast::{
    ConcatKind, Concatenation, Factor, Grouping, Include, StructRule, Term,
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
            Factor::Term(Term::Grouping(Grouping(conc)))
            | Factor::Optional(Term::Grouping(Grouping(conc)))
            | Factor::OneOrMore(Term::Grouping(Grouping(conc)))
            | Factor::ZeroOrMore(Term::Grouping(Grouping(conc))) => {
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

pub fn ast_from_rule_no_alts(StructRule { lhs, rhs }: &StructRule) -> TokenStream2 {
    let ident = proc_macro2::Ident::new(lhs, Span::call_site());
    let Rhs(alts) = rhs;
    let Alternation {
        ref concatenation,
        identifier: _,
    } = alts[0];
    let tupletyp = IncludeAnalysis::concat_to_type(concatenation);
    quote!(#[derive(Debug)]
        struct #ident #tupletyp ;)
}

pub fn ast_from_rule(rule @ StructRule { lhs, rhs }: &StructRule) -> TokenStream2 {
    let ident = Ident::new(lhs, Span::call_site());
    let Rhs(alts) = rhs;
    if alts.len() == 1 {
        ast_from_rule_no_alts(rule)
    } else {
        let enum_variants = alts.iter().enumerate().map(
            |(
                num,
                Alternation {
                    concatenation,
                    identifier,
                },
            ): (usize, &Alternation)| {
                let Concatenation(factors) = concatenation;
                if identifier.is_none() {
                    if let &[ConcatKind::Include(Include(ref factor))] = factors.as_slice() {
                        match factor {
                            Factor::Term(Term::Ident(ident))
                            | Factor::Optional(Term::Ident(ident))
                            | Factor::OneOrMore(Term::Ident(ident))
                            | Factor::ZeroOrMore(Term::Ident(ident)) => {
                                let ident = Ident::new(&ident, Span::call_site());
                                let tupletyp = IncludeAnalysis::concat_to_type(concatenation);
                                quote!(#ident #tupletyp)
                            }
                            _ => panic!("In rule {}, alternation number {} was not given a variant name", ident.to_string(), num + 1),
                        }
                    } else {
                        panic!("In rule {}, alternation number {} was not given a name", ident.to_string(), num + 1)
                    }
                } else {
                    let ident = Ident::new(identifier.as_ref().unwrap(), Span::call_site());
                    let tupletyp = IncludeAnalysis::concat_to_type(concatenation);
                    quote!(#ident #tupletyp)
                }
            },
        );
        quote!(
            #[derive(Debug)]
            enum #ident {
            #(#enum_variants),
               *
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::parser_ast::{Alternation, Rhs};

    use super::super::parser::ast_rule;
    use super::super::parser::parser_ast::StructRule;
    use super::IncludeAnalysis;

    #[test]
    fn analiza_1() {
        let (_, StructRule { lhs: _, rhs }) =
            ast_rule(r#"Kari=>@Numer,"+",@(@(@Plaka, @Jote)?,@"-"+);"#).unwrap();
        let Rhs(alts) = rhs;
        let Alternation {
            ref concatenation,
            identifier: _,
        } = alts[0];
        let analsis = IncludeAnalysis::concat_to_type(concatenation);
        panic!(
            "{:#?}",
            analsis.to_string().split_whitespace().collect::<String>()
        );
    }
}
