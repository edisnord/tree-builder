use tree_builder_macro::rule;

rule!{Test #=> ("term", "asd")}
rule!{Test2 #=> (#d?, "asd"+, Test*, ("a"))?}
rule!{Test3 #=> (#d?, "asd"+, Test*, ("a"?, "ASDA"+))?}
rule!{Test4 #=> (#d?, "asd"+, Test*, ("a"?, "ASDA"+))+}
rule!{Test5 #=> (#d?, "asd"+, Test*, ("a"?, "ASDA"+))*}


fn main(){}
