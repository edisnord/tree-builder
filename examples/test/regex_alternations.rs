use tree_builder_macro::rule;

rule! {Part #=> "a"}

rule!{
    Test #=> "term" | Part? | ("asf", "asdasf")+
}

fn main(){}
