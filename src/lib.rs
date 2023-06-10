pub use tree_builder_macro::build_tree;
pub use tree_builder_macro::regex;
pub mod __private;
pub mod public_parsers;

pub trait Parser{
    fn parse(input: &str) -> nom::IResult<&str, Box<Self>, nom::error::VerboseError<&str>>;
}
