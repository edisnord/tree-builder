use tree_builder_macro::rule;

rule!{
    Test => @"a", @"b", @"c" <Abc>
        |   @#d+, @"a"*, @[asd]? <DigitAASD>
}

fn main(){
    let _ = Test::Abc("a".to_owned(), "b".to_owned(), "c".to_owned());
    let _ = Test::DigitAASD("12321424124".to_owned(), vec!["aaaa".to_owned()], Some("asd".to_owned()));
}
