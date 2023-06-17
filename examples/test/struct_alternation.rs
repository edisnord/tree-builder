use tree_builder_macro::rule;

rule!{
    Test => @"a", @"b", @"c" <Abc>
        |   @#d+, @"a"*, @[asd]? <DigitAASD>
}

fn main(){}
