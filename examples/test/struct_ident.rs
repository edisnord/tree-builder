use tree_builder_macro::rule;

rule!{
    Test => "a"
}

rule!{
    Test2 => @Test, @Test+, @Test*, @Test?
}

fn main(){
    Test2(Box::new(Test()), vec![], vec![], None);
}
