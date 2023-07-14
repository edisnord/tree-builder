use tree_builder_macro::build_tree;

#[derive(Debug)]
pub struct Eval(Box<Add>);
tree_builder::ast_parser_maker!(Eval => @Add);

build_tree! {
    AddAux => "+", @Mult <Addition>
            |  "-", @Mult <Subtraction>;

    MultAux => "*", @Atom <Multiplication>
            | "/", @Atom <Division>;

    Atom => @Number
            | "(", @Add, ")" <Grouping>;

    Number => @Float | @Integer;

    Float #=> #d+, ".", #d+ , EPart?;

    Integer #=> #d+, EPart?;

    EPart #=> ("E" | "e"), ("-" | "+")?, #d+;

    Mult => @Atom, @MultAux*;

    Add => @Mult, @AddAux*;
}

fn main() {}
