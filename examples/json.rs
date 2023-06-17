use tree_builder::{build_tree, Parser};

build_tree! {
    JSON => #s*, @JValue, #s*;

    Number #=> "-"?, ("0" | [1-9], [0-9]*), (".", [0-9]+, EPart?)?;
    EPart #=> ("E" | "e"), ("-" | "+")?, #d+;

    JString #=> "\"",
                    (
                        [^"\\"]
                        | Escape
                    )*,
                "\"";

    Escape #=> "\\", (  "n"
                      | "\\"
                      | "\""
                      | "r"
                      | "t"
                      | "f"
                      | "/"
                      | "u", Hex, Hex, Hex, Hex);

    JValue  =>  @JString
            | @Number
            | @Object
            | @Array
            | "true" <True>
            | "false" <False>
            | "null" <Null>;

    Object => "{", #s*, @ObjectElems?, #s*, "}";
    ObjectElems => @JString, #s*, ":", #s*, @JValue, #s*, @(",", #s*, @ObjectElems)?;
    Hex #=> [0-9a-fA-F];
    Array => "[", #s*, @ArrayElems?, #s*, "]";
    ArrayElems => @JValue, #s*, @(",", #s*, @ArrayElems)?;
}

fn main(){
    let parsed = Array::parse(r#"[ true, -1888300249.8418272, 1140199957, -1319010735, "success", -1523848246.6416917 ]"#);
    let parsed = match parsed {
        Ok((_, out)) => out,
        Err(nom::Err::Error(e)) => panic!("{}", nom::error::convert_error(r#"[ true, -1888300249.8418272, 1140199957, -1319010735, "success", -1523848246.6416917 ]"#, e)),
        _ => panic!("??")
    };
    println!("{:#?}", parsed);

    let input = std::fs::read_to_string("./examples/test.json").unwrap();
    let parsed = JSON::parse(&input);
    let parsed = match parsed {
        Ok((_, out)) => {
            out
        },
        Err(nom::Err::Error(_)) => panic!("{}", "kys"),
        _ => panic!("??")
    };
    println!("{:#?}", parsed);
}
