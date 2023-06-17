#[test]
fn math_expr_parser(){
    let t = trybuild::TestCases::new();
    t.pass("examples/test/math_expr*.rs");
}

#[test]
fn regex_tests(){
    let t = trybuild::TestCases::new();
    t.pass("examples/test/regex*.rs");
}

#[test]
fn struct_tests(){
    let t = trybuild::TestCases::new();
    t.pass("examples/test/struct*.rs");
}

#[test]
fn json_test(){
    let t = trybuild::TestCases::new();
    t.pass("examples/json.rs");
}
