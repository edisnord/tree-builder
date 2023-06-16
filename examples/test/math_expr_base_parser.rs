use tree_builder_macro::rule;

#[derive(Debug)]
pub struct Eval(Box<Add>);
tree_builder::ast_parser_maker!(Eval => @Add);

rule! {Add => @Mult, @AddAux?}

rule! {
    AddAux => "+", @Mult, @AddAux? <Addition>
            |  "-", @Mult, @AddAux? <Subtraction>
}

rule! {Mult => @Atom, @MultAux?;}

rule! {
    MultAux => "*", @Atom, @MultAux? <Multiplication>
            | "/", @Atom, @MultAux? <Division>
}

rule! {
Atom => @Number
     | "(", @Add, ")" <Grouping>
   }

rule! {Number => @Float | @Integer}

rule !(Float #=> #d+, ".", #d+ , EPart?);

rule !(Integer #=> #d+, EPart?);

rule !(EPart #=> ("E" | "e"), ("-" | "+")?, #d+);
fn main(){}
