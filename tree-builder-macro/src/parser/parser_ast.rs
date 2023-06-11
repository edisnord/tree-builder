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

#[derive(PartialEq, Eq, Debug)]
pub enum RuleKind {
    StructRule(StructRule),
    RegexRule(RegRule)
}

/// Representation of a Rule, the main building block of treebuilder's syntax
///
/// In AST generation, the Lhs represents the name of the enum/struct to be generated, and
/// Rhs represents the contents of said struct/enum. A program will generate as many structs/enums
/// as there are rules in a program.
#[derive(PartialEq, Eq, Debug)]
pub struct StructRule {
    pub lhs: String,
    pub rhs: Rhs,
}

/// Representation of a concatenation of many factors in treebuilder's syntax
#[derive(PartialEq, Eq, Debug)]
pub struct Concatenation(pub Vec<ConcatKind>);

#[derive(PartialEq, Eq, Debug)]
pub enum ConcatKind{
    Factor(Factor),
    Include(Include)
}

/// Representation of a factor, which may either be a repetition modifier for a term,
/// or simply a term. In AST generation factors cause terms to be either wrapped in a
/// Vec, an Optional or simply be unwrapped inside a variant.
///
/// In parser generation factors help determine how many times a term may be parsed
#[derive(PartialEq, Eq, Debug)]
pub enum Factor {
    Optional(Term), // opt()
    ZeroOrMore(Term), // many0
    OneOrMore(Term), // many1
    Term(Term),
}

impl Factor {
    pub fn is_grouping(&self) -> bool {
        matches!(self, Factor::Term(Term::Grouping(_))
                     | Factor::Optional(Term::Grouping(_))
                     | Factor::OneOrMore(Term::Grouping(_))
                     | Factor::ZeroOrMore(Term::Grouping(_)))
    }
}


/// A term is the smallest part of treebuilder's syntax, represents either a terminal
/// a grouping, an identifier, inclusion or grouping
///
/// in AST generation only Includes have impact in the amount of elements inside a tuple variant,
/// in parser generation all of the variants of a term have an impact in the selection of the nom parser to be used
/// for parsing
#[derive(PartialEq, Eq, Debug)]
pub enum Term {
    Terminal(Terminal), // tag()
    Grouping(Grouping), // po mendoj ta bej funksion mvete
    Metacharacter(Metacharacter), //The king is back
    Ident(String), // Parser i identit
}

#[derive(PartialEq, Eq, Debug)]
pub struct Include(pub Factor);

#[derive(PartialEq, Eq, Debug)]
pub struct Grouping(pub Box<Concatenation>);

#[derive(PartialEq, Eq, Debug)]
pub struct Rhs(pub Vec<Alternation>);

#[derive(PartialEq, Eq, Debug)]
pub struct Specification(pub Vec<RuleKind>);

#[derive(PartialEq, Eq, Debug)]
pub struct RegRule {
    pub lhs: String,
    pub rhs: RegRhs
}

#[derive(PartialEq, Eq, Debug)]
pub struct RegRhs (pub Vec<RegAlternation>);

#[derive(PartialEq, Eq, Debug)]
pub struct RegAlternation(pub RegConcatenation);

#[derive(PartialEq, Eq, Debug)]
pub struct RegConcatenation(pub Vec<RegFactor>);

#[derive(PartialEq, Eq, Debug)]
pub enum RegFactor {
    Optional(RegTerm), // opt()
    ZeroOrMore(RegTerm), // many0
    OneOrMore(RegTerm), // many1
    Term(RegTerm),
}

#[derive(PartialEq, Eq, Debug)]
pub enum RegTerm {
    Terminal(Terminal), // tag()
    Grouping(RegGrouping), // ketu mendoj duhet hapur new block (ose perdor alt)
    Ident(String), // Parser i identit (Error checking, duhet te jete regex parser sesben)
    Metacharacter(Metacharacter)
}

#[derive(PartialEq, Eq, Debug)]
pub struct RegGrouping(pub Box<RegRhs>, pub Option<String>);

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Metacharacter {
    AllChars, // take(1)
    Digits, // digit()
    NonDigits, // non_digit()
    AlphaNumericUnderscore, // alpha_num_undersc()
    NonAlphaNumericUnderscore, // non_alpha_num_undersc()
    Whitespace,
    NonWhitespace,
    SquareBrackets(Vec<char>),
    ExcludingSquareBrackets(Vec<char>),
}
