use super::parser_ast::*;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while},
    character::complete::char,
    character::complete::one_of,
    character::complete::{alpha1, alphanumeric1, multispace0, multispace1},
    combinator::{map, opt, recognize, value},
    multi::many0_count,
    multi::{many0, separated_list1},
    sequence::pair,
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult, Parser,
};

fn include(input: &str) -> IResult<&str, Include> {
    alt((
        preceded(char('@'), grouping).map(Include::Grouping),
        preceded(char('@'), identifier)
            .map(str::to_owned)
            .map(Include::Ident),
    ))(input)
}

fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))(input)
}

pub fn specification<'a>(input: &'a str) -> IResult<&str, Specification> {
    let skip = move |input: &'a str| -> IResult<&str, Vec<&str>> {
        if input.is_empty() {
            return Err(nom::Err::Error(nom::error::Error::new(
                "",
                nom::error::ErrorKind::Eof,
            )));
        }
        many0(whitespace)(input)
    };

    delimited(
        skip,
        separated_list1(skip, rule).map(Specification),
        opt(skip),
    )(input)
}

fn comment(input: &str) -> IResult<&str, &str> {
    preceded(
        multispace0,
        delimited(
            char('#'),
            take_while(|character| character != '\n' || character == '\0'),
            one_of("\n\0"),
        ),
    )(input)
}

fn field_name(input: &str) -> IResult<&str, &str> {
    delimited(
        pair(tag("<"), multispace0),
        identifier,
        pair(multispace0, tag(">")),
    )(input)
}

fn whitespace(input: &str) -> IResult<&str, &str> {
    alt((comment, multispace1))(input)
}

mod metacharacters {
    use crate::parser::parser_ast::Metacharacter;
    use nom::{
        branch::alt,
        bytes::complete::{tag, take_until},
        character::complete::{char, one_of},
        combinator::value,
        sequence::{delimited, preceded, separated_pair},
        IResult, Parser,
    };

    fn squarebrackets(input: &str) -> IResult<&str, Vec<char>> {
        alt((
            sb_char_range,
            take_until("]").map(str::chars).map(Iterator::collect),
        ))(input)
    }

    fn sb_char_range(input: &str) -> IResult<&str, Vec<char>> {
        let lowercase = "abcdefghijklmnopqrtstuvwxyz";
        let uppercase = "ABCDEFGHIJKLNMNOPQRSTUVXYZ";
        let digit = "0123456789";

        alt((
            separated_pair(one_of(lowercase), char('-'), one_of(lowercase)),
            separated_pair(one_of(uppercase), char('-'), one_of(uppercase)),
            separated_pair(one_of(digit), char('-'), one_of(digit)),
        ))
        .map(|(bot, top)| -> Vec<char> {
            let range = bot..=top;
            range.into_iter().collect()
        })
        .parse(input)
    }

    pub fn metacharacter(input: &str) -> IResult<&str, Metacharacter> {
        use Metacharacter::*;
        alt((
            preceded(
                char('\\'),
                alt((
                    value(AlphaNumericUnderscore, char('w')),
                    value(NonAlphaNumericUnderscore, char('W')),
                    value(Whitespace, char('s')),
                    value(NonWhitespace, char('S')),
                    value(Digits, char('d')),
                    value(NonDigits, char('D')),
                )),
            ),
            delimited(tag("[^"), squarebrackets, char(']')).map(ExcludingSquareBrackets),
            delimited(char('['), squarebrackets, char(']')).map(SquareBrackets),
            char('.').map(|_| AllChars),
        ))(input)
    }
}

fn term(input: &str) -> IResult<&str, Term> {
    alt((
        map(include, Term::Include),
        map(grouping, Term::Grouping),
        map(terminal::terminal, Term::Terminal),
        map(identifier, |ident| Term::Ident(ident.to_string())),
        map(metacharacters::metacharacter, Term::Metacharacter),
    ))(input)
}

fn factor(input: &str) -> IResult<&str, Factor> {
    alt((
        map(terminated(term, tag("?")), Factor::Optional),
        map(terminated(term, tag("*")), Factor::ZeroOrMore),
        map(terminated(term, tag("+")), Factor::OneOrMore),
        map(term, Factor::Term),
    ))(input)
}

fn grouping(input: &str) -> IResult<&str, Grouping> {
    delimited(
        pair(char('('), multispace0),
        rhs,
        pair(multispace0, char(')')),
    )
    .map(Box::new)
    .map(Grouping)
    .parse(input)
}

fn rhs(input: &str) -> IResult<&str, Rhs> {
    alternations.map(Rhs).parse(input)
}

fn lhs(input: &str) -> IResult<&str, &str> {
    identifier(input)
}

fn rule<'a>(input: &'a str) -> IResult<&str, Rule> {
    let rule_separator = |input: &'a str| -> IResult<&str, &str> {
        delimited(multispace0, tag("=>"), multispace0)(input)
    };

    terminated(
        delimited(
            multispace0,
            separated_pair(lhs, rule_separator, rhs).map(|(lhs, rhs)| Rule {
                lhs: lhs.to_string(),
                rhs,
            }),
            multispace0,
        ),
        char(';'),
    )(input)
}

fn alternation_body(input: &str) -> IResult<&str, (Concatenation, Option<&str>)> {
    separated_pair(concatenation, multispace0, opt(field_name))(input)
}

fn alternation(input: &str) -> IResult<&str, Alternation> {
    separated_pair(concatenation, multispace0, opt(field_name))(input).map(
        |(rem, (concatenation, identifier))| {
            (
                rem,
                Alternation {
                    concatenation,
                    identifier: identifier.map(str::to_string),
                },
            )
        },
    )
}

fn alternations(input: &str) -> IResult<&str, Vec<Alternation>> {
    alt((
        separated_list1(alternation_separator, alternation),
        alternation.map(|x| vec![x]),
    ))(input)
}

fn alternation_separator(input: &str) -> IResult<&str, char> {
    delimited(multispace0, char('|'), multispace0)(input)
}

fn concatenation(input: &str) -> IResult<&str, Concatenation> {
    delimited(
        multispace0,
        alt((
            separated_list1(concat_separator, factor),
            factor.map(|result| vec![result]),
        )),
        multispace0,
    )(input)
    .map(|(rem, output)| (rem, Concatenation(output)))
}

fn concat_separator(input: &str) -> IResult<&str, char> {
    delimited(multispace0, char(','), multispace0)(input)
}

mod terminal {
    use nom::branch::alt;
    use nom::bytes::streaming::is_not;
    use nom::character::streaming::{char, multispace1};
    use nom::combinator::{map, value, verify};
    use nom::error::FromExternalError;
    use nom::error::ParseError;
    use nom::multi::fold_many0;
    use nom::sequence::{delimited, preceded};
    use nom::IResult;

    use crate::parser::parser_ast::Terminal;

    fn parse_escaped_char<'a, E>(input: &'a str) -> IResult<&'a str, char, E>
    where
        E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
    {
        preceded(
            char('\\'),
            alt((
                value('\n', char('n')),
                value('\r', char('r')),
                value('\t', char('t')),
                value('\u{08}', char('b')),
                value('\u{0C}', char('f')),
                value('\\', char('\\')),
                value('\"', char('\"')),
            )),
        )(input)
    }

    fn parse_escaped_whitespace<'a, E: ParseError<&'a str>>(
        input: &'a str,
    ) -> IResult<&'a str, &'a str, E> {
        preceded(char('\\'), multispace1)(input)
    }

    fn parse_literal<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
        let not_quote_slash = is_not("\"\\");

        verify(not_quote_slash, |s: &str| !s.is_empty())(input)
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum StringFragment<'a> {
        Literal(&'a str),
        EscapedChar(char),
        EscapedWS,
    }

    fn parse_fragment<'a, E>(input: &'a str) -> IResult<&'a str, StringFragment<'a>, E>
    where
        E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
    {
        alt((
            // The `map` combinator runs a parser, then applies a function to the output
            // of that parser.
            map(parse_literal, StringFragment::Literal),
            map(parse_escaped_char, StringFragment::EscapedChar),
            value(StringFragment::EscapedWS, parse_escaped_whitespace),
        ))(input)
    }

    pub fn terminal<'a>(input: &'a str) -> IResult<&'a str, Terminal> {
        let build_string = fold_many0(parse_fragment, String::new, |mut string, fragment| {
            match fragment {
                StringFragment::Literal(s) => string.push_str(s),
                StringFragment::EscapedChar(c) => string.push(c),
                StringFragment::EscapedWS => {}
            }
            string
        });

        delimited(char('"'), build_string, char('"'))(input)
            .map(|(rest, terminal)| (rest, Terminal(terminal)))
    }
}

#[cfg(test)]
mod tests {
    use super::super::parsing::terminal::terminal;
    use super::metacharacters::metacharacter;
    use super::*;

    #[test]
    fn test_terminal() {
        let (_, output) = terminal(r#""aaa""#).unwrap();
        assert_eq!(output.0, "aaa".to_owned());
    }

    #[test]
    fn test_field_name() {
        let (_, output) = super::field_name("<fn>").unwrap();
        assert!(output.eq("fn"));
        let (_, output) = field_name("<  fn  >").unwrap();
        assert!(output.eq("fn"));
    }

    #[test]
    fn test_term() {
        let (_, output) = term("Fizz").unwrap();
        if let Term::Ident(str) = output {
            assert_eq!(str, "Fizz".to_string())
        } else {
            panic!()
        }
        let (_, output) = term(r#""Fizz""#).unwrap();
        if let Term::Terminal(Terminal(str)) = output {
            assert_eq!(str, "Fizz".to_string())
        } else {
            panic!()
        }
    }

    #[test]
    fn test_factor() {
        let (_, output) = factor("Fizz?").unwrap();
        if let Factor::Optional(term1) = output {
            let (_, term2) = term("Fizz").unwrap();
            assert_eq!(term1, term2)
        }
        let (_, output) = factor("Fizz+").unwrap();
        if let Factor::OneOrMore(term1) = output {
            let (_, term2) = term("Fizz").unwrap();
            assert_eq!(term1, term2)
        }
        let (_, output) = factor("Fizz*").unwrap();
        if let Factor::ZeroOrMore(term1) = output {
            let (_, term2) = term("Fizz").unwrap();
            assert_eq!(term1, term2)
        }
        let (_, output) = factor("Fizz").unwrap();
        if let Factor::Term(term1) = output {
            let (_, term2) = term("Fizz").unwrap();
            assert_eq!(term1, term2)
        }
    }

    #[test]
    fn test_concatenation() {
        let (_, output) = concatenation("Fizz?, Buzz").unwrap();
        let Concatenation(factors) = output;
        let test_factors: Vec<Factor> = vec![factor("Fizz?"), factor("Buzz")]
            .into_iter()
            .map(|x| x.unwrap().1)
            .collect();
        assert_eq!(factors, test_factors)
    }

    #[test]
    fn test_alternation() {
        let expected = vec![Alternation {
            concatenation: concatenation("Fizz, Buzz").unwrap().1,
            identifier: Some("bazz".to_owned()),
        }];

        assert_eq!(expected, alternations("Fizz, Buzz <bazz>").unwrap().1);

        let expected = vec![
            Alternation {
                concatenation: concatenation("Fizz, Buzz").unwrap().1,
                identifier: Some("bazz".to_owned()),
            },
            Alternation {
                concatenation: concatenation("Fizz2, Buzz2").unwrap().1,
                identifier: Some("bazz2".to_owned()),
            },
        ];

        assert_eq!(
            expected,
            alternations("Fizz, Buzz <bazz> | Fizz2, Buzz2 <bazz2>")
                .unwrap()
                .1
        );
    }

    #[test]
    fn test_metacharacters() {
        let output = metacharacter(r#"[^"\]"#).unwrap().1;
        assert_eq!(
            Metacharacter::ExcludingSquareBrackets(vec!['"', '\\']),
            output
        );
        let output = metacharacter(r#"["\]"#).unwrap().1;
        assert_eq!(Metacharacter::SquareBrackets(vec!['"', '\\']), output);
        let output = delimited(tag("aaa"), metacharacter, tag("bbb"))(r#"aaa.bbb"#)
            .unwrap()
            .1;
        assert_eq!(Metacharacter::AllChars, output);
        
    }

    #[test]
    fn test_specification() {
        let source = r#"# Basic expression definition in Lox
        Expression => Literal | Unary | Binary | Grouping ;

        # Grouping of expressions
        Grouping   => "(" , Expression , ")"; # Inline comment

        Number     => Integer <Integer> 
                    | Float <Float> ;
        Literal    => "\"", @([^"\]*, ("\\",.,[^"\]*)*), "\"" <String>| Number <Numeric> | "true"<True> | "false" <False> | "nil" <Nil> ;
        
        BinaryOp   => "+" <Plus> | "-" <Minus>
                    | "*" <Star> | "/" <Slash>
                    | "=" <Equal> |"==" <EqualEqual>
                    | "!" <Bang> |"!=" <BangEqual>
                    | "<" <Less> | ">=" <LessEqual> ;
        
        Binary     => Expression, BinaryOp, Expression ;
        
        
        UnaryOp    => "-" <Minus> | "!" <Bang> ;
        Unary      => UnaryOp, Expression ;
        
        Float    => ("-" | "+")?, \d+, (".", \d+)? , (("E" | "e"), ("-" | "+")?, \d+)? ;
        Integer  => "-"?, [0-9]+, ("E", ("-" | "+"), [0-a]+)? ;"#;

        let output = specification(source).unwrap().1;
        println!("{:#?}", output);
        let expected = Specification(
            vec![
                rule("Expression => Literal | Unary | Binary | Grouping ;"),
                rule(r#"Grouping   => "(" , Expression , ")";"#),
                rule(r#"Number     => Integer <Integer> 
                                    | Float <Float> ;"#),
                rule(r#"Literal    => "\"", @([^"\]*, ("\\",.,[^"\]*)*), "\"" <String>| Number <Numeric> | "true"<True> | "false" <False> | "nil" <Nil> ;"#),
                rule(r#"BinaryOp   => "+" <Plus> | "-" <Minus>
                | "*" <Star> | "/" <Slash>
                | "=" <Equal> |"==" <EqualEqual>
                | "!" <Bang> |"!=" <BangEqual>
                | "<" <Less> | ">=" <LessEqual> ;"#),
                rule(r#"Binary => Expression, BinaryOp, Expression ;"#),
                rule(r#"UnaryOp    => "-" <Minus> | "!" <Bang> ;"#),
                rule(r#"Unary      => UnaryOp, Expression ;"#),
                rule(r#"Float    => ("-" | "+")?, \d+, (".", \d+)? , (("E" | "e"), ("-" | "+")?, \d+)? ;"#),
                rule(r#"Integer  => "-"?, [0-9]+, ("E", ("-" | "+"), [0-a]+)? ;"#)
            ].into_iter().map(|a| a.unwrap().1).collect()
        );

        assert_eq!(expected, output);
    }
}
