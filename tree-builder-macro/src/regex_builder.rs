use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};

use crate::parser::parser_ast::{RegAlternation, RegGrouping, RegRhs, RegRule, RegTerm};

pub fn owned_push() -> TokenStream2 {
    quote! {result.push(res.as_str());}
}

pub fn standard_push() -> TokenStream2 {
    quote! {result.push(res);}
}

pub mod term {
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
        macro_rules! metachar {
                ($fun:ident) => {
                    quote! {tree_builder::public_parsers::$fun::<_, tree_builder::__private::nom::error::VerboseError<&str>>()}
                };
            }
        let complete = quote! {tree_builder::__private::nom::character::complete::};
        match metachar {
            Metacharacter::Digits => metachar!(digit),
            Metacharacter::AllChars => metachar!(all_chars),
            Metacharacter::NonDigits => metachar!(non_digit),
            Metacharacter::Whitespace => metachar!(whitespace),
            Metacharacter::NonWhitespace => metachar!(non_whitespace),
            Metacharacter::AlphaNumericUnderscore => metachar!(alpha_num_underscore),
            Metacharacter::NonAlphaNumericUnderscore => metachar!(non_alpha_num_underscore),
            Metacharacter::SquareBrackets(values) => {
                let list: String = values.into_iter().collect();
                let literal = LitStr::new(&list, Span::call_site());
                quote! {#complete one_of::<_, _, nom::error::VerboseError<&str>>(#literal)}
            }
            Metacharacter::ExcludingSquareBrackets(values) => {
                let list: String = values.into_iter().collect();
                let literal = LitStr::new(&list, Span::call_site());
                quote! {#complete none_of::<_, _, nom::error::VerboseError<&str>>(#literal)}
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
}

pub mod factor {
    use crate::parser::parser_ast::{RegFactor, RegTerm};
    use proc_macro2::TokenStream as TokenStream2;
    use quote::{format_ident, quote};

    pub fn option_invocation(opt: &RegFactor) -> TokenStream2 {
        match opt {
            RegFactor::Optional(term) => {
                let term = super::term::term(term);
                quote! {tree_builder::__private::nom::combinator::opt(#term)}
            }
            a => panic!(
                "Unexpected behavior, please report this error, entered option_invoc, was {:#?}",
                a
            ),
        }
    }

    pub fn option_post(opt: &RegFactor) -> TokenStream2 {
        match opt {
            RegFactor::Optional(term) => {
                let (decl, unwrap) = match term {
                    RegTerm::Ident(ident) => {
                        let ident = format_ident!("{}", ident);
                        (
                            quote! {let #ident (res)},
                            quote! {*res.unwrap_or_else(|| Box::new(#ident (String::new())))},
                        )
                    }
                    _ => (
                        quote! {let res},
                        quote! {res.unwrap_or_else(||String::new())},
                    ),
                };
                quote! {#decl = #unwrap ;}
            }
            a => panic!(
                "Unexpected behavior, please report this error, entered option_post, was {:#?}",
                a
            ),
        }
    }

    pub fn many1_invocation(opt: &RegFactor) -> TokenStream2 {
        match opt {
            RegFactor::OneOrMore(term) => {
                let term = super::term::term(term);
                quote! {tree_builder::__private::nom::multi::many1(#term)}
            }
            a => panic!(
                "Unexpected behavior, please report this error, entered many1_invoc, was {:#?}",
                a
            ),
        }
    }

    pub fn many0_invocation(opt: &RegFactor) -> TokenStream2 {
        match opt {
            RegFactor::ZeroOrMore(term) => {
                let term = super::term::term(term);
                quote! {tree_builder::__private::nom::multi::many0(#term)}
            }
            a => panic!(
                "Unexpected behavior, please report this error, entered many0_invoc, was {:#?}",
                a
            ),
        }
    }

    pub fn term_invocation(term: &RegFactor) -> TokenStream2 {
        match term {
            RegFactor::Term(term) => super::term::term(term),
            a => panic!(
                "Unexpected behavior, please report this error, entered term, was {:#?}",
                a
            ),
        }
    }

    pub fn many_post(many: &RegFactor) -> TokenStream2 {
        match many {
            RegFactor::OneOrMore(term) | RegFactor::ZeroOrMore(term) => match term {
                RegTerm::Ident(ident) => {
                    let ident = format_ident!("{}", ident);
                    quote! {let res = res.into_iter().map(|res|{
                        let res = *res;
                        let #ident (res) = res;
                        res
                    }).collect::<std::vec::Vec<std::string::String>>().join("");}
                }
                _ => quote! {let res = res.join("");},
            },
            a => panic!(
                "Unexpected behavior, please report this error, entered many_post, was {:#?}",
                a
            ),
        }
    }
}

pub mod concatenation {
    use crate::parser::parser_ast::{RegConcatenation, RegFactor, RegTerm};
    use proc_macro2::TokenStream as TokenStream2;
    use quote::quote;

    pub fn concatenation(RegConcatenation(concs): &RegConcatenation) -> TokenStream2 {
        let declaration = quote! {let (input, res) = };
        let mapper = |factor: &RegFactor| match factor {
            RegFactor::Term(t) => {
                let push = if matches!(t, RegTerm::Grouping(_)) {
                    super::owned_push()
                } else {
                    super::standard_push()
                };
                let invocation = super::factor::term_invocation(factor);
                quote! {
                    #declaration #invocation(input)?;
                    #push
                }
            }
            RegFactor::Optional(_) => {
                let invocation = super::factor::option_invocation(factor);
                let post = super::factor::option_post(factor);
                let push = super::owned_push();
                quote! {
                    #declaration #invocation(input)?;
                    #post
                    #push
                }
            }
            RegFactor::ZeroOrMore(_) => {
                let invocation = super::factor::many0_invocation(factor);
                let post = super::factor::many_post(factor);
                let push = super::owned_push();
                quote! {
                    #declaration #invocation(input)?;
                    #post
                    #push
                }
            }
            RegFactor::OneOrMore(_) => {
                let invocation = super::factor::many1_invocation(factor);
                let post = super::factor::many_post(factor);
                let push = super::owned_push();
                quote! {
                    #declaration #invocation(input)?;
                    #post
                    #push
                }
            }
        };
        let concs: Vec<TokenStream2> = concs.into_iter().map(mapper).collect();
        quote! {
            #(#concs)
            *
        }
    }
}

pub fn alternation(RegAlternation(conc): &RegAlternation) -> TokenStream2 {
    let concat = concatenation::concatenation(conc);
    quote! {
        #concat
    }
}

pub fn righths<'a>(rhs: &'a mut RegRhs) -> TokenStream2 {
    let gr = analyze_groupings(rhs);
    let alts: &mut Vec<RegAlternation> = rhs.0.as_mut();
    let alt = if alts.len() == 1 {
        alternation(alts.get(0).unwrap())
    } else {
        let name_mapper =
            |(i, alt): (usize, &'a RegAlternation)| -> (TokenStream2, &'a RegAlternation) {
                let i = syn::Index::from(i);
                let name = format_ident!("alt{}", i);
                (quote! {#name}, alt)
            };

        let closure_mapper =
            |(name, RegAlternation(conc)): (TokenStream2, &'a RegAlternation)| -> TokenStream2 {
                let body = concatenation::concatenation(conc);
                quote! {
                    let #name = |input| {
                                    let mut result: std::vec::Vec<&str> = std::vec::Vec::new();
                                    #body
                                    let mut result = result.join("");
                                    Ok((input, result))
                                };
                }
            };

        let closures: Vec<TokenStream2> = alts
            .iter()
            .enumerate()
            .map(name_mapper)
            .map(closure_mapper)
            .collect();

        let names: Vec<TokenStream2> = alts.iter().enumerate().map(|a| name_mapper(a).0).collect();
        let push = owned_push();
        quote! {
            #(#closures)
              *

            let (input, res) = tree_builder::__private::nom::branch::alt((#(#names),*))(input)?;
            #push
        }
    };
    quote! {
        let mut result: std::vec::Vec<&str> = std::vec::Vec::new();
        #gr
        #alt
        let mut result = result.join("");
    }
}

pub fn rule(RegRule { lhs, rhs }: &mut RegRule) -> TokenStream2 {
    let rhs = righths(rhs);
    let ident = format_ident!("{}", lhs);
    quote! {
        #[derive(Debug)]
        struct #ident(String);

        impl tree_builder::Parser for #ident {
            fn parse(input: &str) -> tree_builder::__private::nom::IResult<&str, std::boxed::Box<Self>, tree_builder::__private::nom::error::VerboseError<&str>> {
                #rhs
                Ok((input, std::boxed::Box::new(Self(result))))
            }
        }
    }
}

fn analyze_groupings<'a>(RegRhs(alts): &'a mut RegRhs) -> TokenStream2 {
    use crate::parser::parser_ast::RegFactor;
    let mut closures: Vec<TokenStream2> = vec![];
    let mut count: usize = 0usize;
    for alt in alts {
        let conc = &mut (alt.0);
        for factor in &mut (conc.0) {
            let term = match factor {
                RegFactor::Term(ref mut term) => term,
                RegFactor::Optional(ref mut term) => term,
                RegFactor::OneOrMore(ref mut term) => term,
                RegFactor::ZeroOrMore(ref mut term) => term,
            };
            if let RegTerm::Grouping(ref mut group @ RegGrouping(_, _)) = term {
                group.1 = Some(format!("group{}", count));
                count = count + 1usize;
                let rhs = righths(group.0.as_mut());
                let ident = format_ident!("{}", group.1.as_ref().unwrap());
                let closure = quote! {
                    let #ident: fn(&str) -> tree_builder::__private::nom::IResult<&str, String, tree_builder::__private::nom::error::VerboseError<&str>> =
                    |input| {
                            let mut result: std::vec::Vec<&str> = std::vec::Vec::new();
                            #rhs
                            Ok((input, result))
                    };
                };
                closures.push(closure);
            };
        }
    }
    quote! {#(#closures)
    *}
}
