use tree_builder_macro::rule;


rule!{Test3 => @Test1}
rule!{Test4 => @Test1, @Test2}
rule!{Test5 => @(@Test1, @Test2), @Test3}
rule!{Test2 => @#d}
rule!{Test1 => @"a"}
rule!{
    Rule => @#d+, @"a"*, @[a-z]?
}
fn main(){}
