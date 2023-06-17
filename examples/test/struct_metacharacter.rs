use tree_builder_macro::rule;
macro_rules! mk_test {
    ($x:tt) => {
        rule!{
            Test$x => #$x, #$x+, #$x*, #$x?
        }
    };
}
macro_rules! mk_sqbr_test {
    ($name:ident, $x:tt) => {
        rule!{
            Test$name => $x+, $x*, $x, $x?
        }
    };
}

mk_test!{D}
mk_test!{d}
mk_test!{S}
mk_test!{s}
mk_test!{W}
mk_test!{w}
rule!{
    TestPoint => .+, .*, .?, .
}
mk_sqbr_test!{az, [a-z]}
mk_sqbr_test!{AZ, [A-Z]}
mk_sqbr_test!{AZaz, [a-zA-Z]}
mk_sqbr_test!{nonAZaz, [^a-zA-Z]}
mk_sqbr_test!{nonabc, [^abc]}

fn main(){}
