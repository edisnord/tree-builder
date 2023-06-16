use super::IncludeAnalysis;
use crate::parser::parser_ast::{
    Alternation, ConcatKind, Concatenation, Factor, Include, Rhs, StructRule, Term,
};
use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::{Ident, Span};
use quote::quote;

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
                            _ => panic!(
                                "In rule {}, alternation number {} was not given a variant name",
                                ident.to_string(),
                                num + 1
                            ),
                        }
                    } else {
                        panic!(
                            "In rule {}, alternation number {} was not given a name",
                            ident.to_string(),
                            num + 1
                        )
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
