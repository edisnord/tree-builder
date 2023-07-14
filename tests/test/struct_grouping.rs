use tree_builder_macro::rule;

rule!{
    Test => "b"
}

rule!{
    Test2 => ("c"*, Test+, ("a", "b", #d))
}

rule!{Test3 => (#d?, "asd"+, Test*, ("a"?, "ASDA"+)+)?}
rule!{Test4 => (#d?, "asd"+, Test*, ("a"?, "ASDA"+))+}
rule!{Test5 => (#d?, "asd"+, Test*, ("a"?, "ASDA"+))*}

fn main(){
}
