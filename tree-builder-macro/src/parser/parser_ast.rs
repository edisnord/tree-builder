/// Representation of a terminal in treebuilder syntax
///
/// Terminal example: "Terminal" -> Terminal("Terminal".to_owned())
///
/// Cannot be included using the @ syntax so has no impact on the AST generation, but does have impact in parser generation
#[derive(PartialEq, Eq, Debug)]
pub struct Terminal(pub String);

/// Representation of an Alternation in treebuilder syntax
///
/// Alternation example: |? Concatenation1 \<Alternation1\> |?
///
/// becomes:
/// ```
/// Alternation{
///     concatenation: Concatenation1,
///     identifier: Some(Alternation1)
/// }
/// ```
/// If there is only one alternation in a Rhs, the AST node generated will be a tuple struct,
/// but if there is more than one alternations are equivalent to an enum's variants, with their
/// names being given inside \<\> tags in the alternation. You cannot have alternations without
/// identifiers if there are more than one alternations.                                                      
#[derive(PartialEq, Eq, Debug)]
pub struct Alternation {
    pub concatenation: Concatenation,
    pub identifier: Option<String>,
}

/// Representation of a Rule, the main building block of treebuilder's syntax
///
/// In AST generation, the Lhs represents the name of the enum/struct to be generated, and
/// Rhs represents the contents of said struct/enum. A program will generate as many structs/enums
/// as there are rules in a program.
#[derive(PartialEq, Eq, Debug)]
pub struct Rule {
    pub lhs: String,
    pub rhs: Rhs,
}

/// Representation of a concatenation of many factors in treebuilder's syntax
#[derive(PartialEq, Eq, Debug)]
pub struct Concatenation(pub Vec<Factor>);

/// Representation of a factor, which may either be a repetition modifier for a term,
/// or simply a term. In AST generation factors cause terms to be either wrapped in a
/// Vec, an Optional or simply be unwrapped inside a variant.
///
/// In parser generation factors help determine how many times a term may be parsed
#[derive(PartialEq, Eq, Debug)]
pub enum Factor {
    Optional(Term),
    ZeroOrMore(Term),
    OneOrMore(Term),
    Term(Term),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Metacharacter {
    AllChars,
    Digits,
    NonDigits,
    AlphaNumericUnderscore,
    NonAlphaNumericUnderscore,
    Whitespace,
    NonWhitespace,
    SquareBrackets(Vec<char>),
    ExcludingSquareBrackets(Vec<char>),
}

/// A term is the smallest part of treebuilder's syntax, represents either a terminal
/// a grouping, an identifier, inclusion or grouping
/// 
/// in AST generation only Includes have impact in the amount of elements inside a tuple variant,
/// in parser generation all of the variants of a term have an impact in the selection of the nom parser to be used
/// for parsing
#[derive(PartialEq, Eq, Debug)]
pub enum Term {
    Metacharacter(Metacharacter),
    Terminal(Terminal),
    Grouping(Grouping),
    Ident(String),
    Include(Include),
}


#[derive(PartialEq, Eq, Debug)]
pub enum Include {
    Grouping(Grouping),
    Ident(String),
}

#[derive(PartialEq, Eq, Debug)]
pub struct Grouping(pub Box<Rhs>);

#[derive(PartialEq, Eq, Debug)]
pub struct Rhs(pub Vec<Alternation>);

#[derive(PartialEq, Eq, Debug)]
pub struct Specification(pub Vec<Rule>);
