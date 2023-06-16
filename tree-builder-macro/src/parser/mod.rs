pub mod parser_ast;
mod parsing;

pub use parsing::specification;
pub use parsing::regex::rule as reg_rule;
pub use parsing::struct_rule;
pub use parsing::rule as rule;
