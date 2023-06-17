pub use tree_builder_macro::build_tree;
pub use tree_builder_macro::rule;
pub use tree_builder_macro::ast_parser_maker;
pub use __private::nom::error::convert_error;
pub mod __private;
pub mod public_parsers;

pub trait Parser{
    fn parse(input: &str) -> nom::IResult<&str, Box<Self>, nom::error::VerboseError<&str>>;
}
