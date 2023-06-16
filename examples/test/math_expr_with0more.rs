use tree_builder_macro::rule;

#[derive(Debug)]
pub struct Eval(Box<Add>);
tree_builder::ast_parser_maker!(Eval => @Add);

rule! {
    AddAux => "+", @Mult <Addition>
            |  "-", @Mult <Subtraction>
}

rule! {
    MultAux => "*", @Atom <Multiplication>
            | "/", @Atom <Division>
}

rule! {
Atom => @Number
     | "(", @Add, ")" <Grouping>
   }

rule! {Number => @Float | @Integer}

rule!(Float #=> #d+, ".", #d+ , EPart?);

rule!(Integer #=> #d+, EPart?);

rule!(EPart #=> ("E" | "e"), ("-" | "+")?, #d+);

rule! {Mult => @Atom, @MultAux*}

rule! {Add => @Mult, @AddAux*}
fn main(){}
