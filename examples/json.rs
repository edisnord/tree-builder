use std::collections::HashMap;
use tree_builder::{build_tree, Parser};

const TEST1: &str = r#"
[
  {
    "_id": "64b13dbc4848c3a4c22284c7",
    "index": 0,
    "guid": "2acbba8f-939c-4266-a2a2-cc44fc4f1bfb",
    "isActive": true,
    "balance": "$3,212.68",
    "picture": "http://placehold.it/32x32",
    "age": 29,
    "eyeColor": "blue",
    "name": "Alice Sargent",
    "gender": "female",
    "company": "INEAR",
    "email": "alicesargent@inear.com",
    "phone": "+1 (974) 407-3296",
    "address": "369 Colin Place, Falconaire, Alabama, 3946",
    "about": "Eu duis ad veniam magna reprehenderit ea irure consequat quis nulla ex velit sit esse. Dolore eu sit qui ad. Aute amet consequat sit aliquip pariatur eiusmod irure cillum do. Ex irure veniam ad fugiat fugiat.\r\n",
    "registered": "2014-11-09T05:06:52 -01:00",
    "latitude": 33.061942,
    "longitude": 175.617123,
    "tags": [
      "consectetur",
      "reprehenderit",
      "non",
      "do",
      "dolore",
      "Lorem",
      "eiusmod"
    ],
    "friends": [
      {
        "id": 0,
        "name": "Harrison Clayton"
      },
      {
        "id": 1,
        "name": "Chapman Bonner"
      },
      {
        "id": 2,
        "name": "Perez Alston"
      }
    ],
    "greeting": "Hello, Alice Sargent! You have 3 unread messages.",
    "favoriteFruit": "strawberry"
  },
  {
    "_id": "64b13dbc6b842506323ed605",
    "index": 1,
    "guid": "af395b6b-abb7-4c25-83da-016c6af99ff4",
    "isActive": true,
    "balance": "$2,568.10",
    "picture": "http://placehold.it/32x32",
    "age": 33,
    "eyeColor": "blue",
    "name": "Marian Rosales",
    "gender": "female",
    "company": "ORBIN",
    "email": "marianrosales@orbin.com",
    "phone": "+1 (877) 416-2903",
    "address": "324 Howard Alley, Bayview, Utah, 5811",
    "about": "Aliquip elit nulla cupidatat fugiat adipisicing. Qui ea est aliquip duis do est ut magna irure do incididunt. Ipsum quis eiusmod ad aliquip officia. Elit nostrud ea anim incididunt cupidatat voluptate ad id. Veniam anim ut non nisi ut voluptate. Ea in nisi mollit veniam.\r\n",
    "registered": "2023-06-13T01:15:46 -02:00",
    "latitude": -16.370648,
    "longitude": -31.48681,
    "tags": [
      "in",
      "tempor",
      "do",
      "sit",
      "esse",
      "reprehenderit",
      "amet"
    ],
    "friends": [
      {
        "id": 0,
        "name": "Taylor Holland"
      },
      {
        "id": 1,
        "name": "Manuela Sims"
      },
      {
        "id": 2,
        "name": "Nita Nichols"
      }
    ],
    "greeting": "Hello, Marian Rosales! You have 6 unread messages.",
    "favoriteFruit": "strawberry"
  },
  {
    "_id": "64b13dbc84d4ca298c701f83",
    "index": 2,
    "guid": "51f9b01e-b915-4d02-be07-4e2852c552e5",
    "isActive": true,
    "balance": "$1,478.54",
    "picture": "http://placehold.it/32x32",
    "age": 38,
    "eyeColor": "green",
    "name": "Holcomb Pickett",
    "gender": "male",
    "company": "INTRAWEAR",
    "email": "holcombpickett@intrawear.com",
    "phone": "+1 (842) 563-3307",
    "address": "432 Lloyd Street, Matthews, Oklahoma, 7005",
    "about": "Magna velit laborum adipisicing ut ex sunt cupidatat. Mollit id eu eu Lorem eu aliquip pariatur laboris fugiat laborum excepteur dolor. Mollit officia laboris laborum tempor. Nisi pariatur deserunt culpa duis est excepteur eiusmod aliquip. Incididunt ad non mollit nisi amet consequat elit exercitation exercitation tempor. Cillum deserunt in Lorem in quis deserunt ut qui adipisicing. Non minim nisi duis adipisicing ad nulla quis.\r\n",
    "registered": "2021-05-22T09:39:01 -02:00",
    "latitude": -13.40349,
    "longitude": -151.844763,
    "tags": [
      "in",
      "tempor",
      "exercitation",
      "veniam",
      "laborum",
      "officia",
      "id"
    ],
    "friends": [
      {
        "id": 0,
        "name": "Atkinson Willis"
      },
      {
        "id": 1,
        "name": "Slater Roach"
      },
      {
        "id": 2,
        "name": "Miriam Holman"
      }
    ],
    "greeting": "Hello, Holcomb Pickett! You have 1 unread messages.",
    "favoriteFruit": "banana"
  },
  {
    "_id": "64b13dbcf413340c5d3189ee",
    "index": 3,
    "guid": "05d93e0d-4463-425f-b3cd-bd79808c3e0a",
    "isActive": true,
    "balance": "$1,160.99",
    "picture": "http://placehold.it/32x32",
    "age": 22,
    "eyeColor": "green",
    "name": "Delores Malone",
    "gender": "female",
    "company": "SNOWPOKE",
    "email": "deloresmalone@snowpoke.com",
    "phone": "+1 (951) 565-2959",
    "address": "712 McKinley Avenue, Brewster, Rhode Island, 378",
    "about": "Elit ea officia dolor quis ipsum commodo elit. Fugiat labore do ad eiusmod laboris ipsum. Adipisicing nulla do officia do excepteur sunt aliqua excepteur incididunt labore ipsum.\r\n",
    "registered": "2022-05-16T11:29:21 -02:00",
    "latitude": 17.271941,
    "longitude": 27.991326,
    "tags": [
      "cupidatat",
      "eiusmod",
      "culpa",
      "aliquip",
      "nulla",
      "nulla",
      "proident"
    ],
    "friends": [
      {
        "id": 0,
        "name": "Fletcher Tate"
      },
      {
        "id": 1,
        "name": "Myra Casey"
      },
      {
        "id": 2,
        "name": "Herman Bowman"
      }
    ],
    "greeting": "Hello, Delores Malone! You have 8 unread messages.",
    "favoriteFruit": "strawberry"
  },
  {
    "_id": "64b13dbc93408c717d1a7524",
    "index": 4,
    "guid": "edac1510-15c2-4bf8-8ebd-bd8889d12460",
    "isActive": true,
    "balance": "$2,083.48",
    "picture": "http://placehold.it/32x32",
    "age": 38,
    "eyeColor": "brown",
    "name": "Priscilla Albert",
    "gender": "female",
    "company": "JUMPSTACK",
    "email": "priscillaalbert@jumpstack.com",
    "phone": "+1 (997) 411-3544",
    "address": "958 Vandam Street, Osmond, California, 8773",
    "about": "Consequat in eu officia enim sunt nulla in dolore. Cillum consequat veniam minim labore est sunt aute. Pariatur pariatur Lorem cillum adipisicing ex exercitation. Irure aute ullamco dolor enim ullamco qui. Occaecat amet elit proident aliquip eu et nostrud non incididunt deserunt officia ullamco. Magna sunt sunt do irure consequat excepteur sunt consectetur ad. Sint mollit eu incididunt deserunt sunt laborum consectetur do aliquip.\r\n",
    "registered": "2019-04-03T07:52:13 -02:00",
    "latitude": -51.763731,
    "longitude": 65.890049,
    "tags": [
      "sit",
      "pariatur",
      "ipsum",
      "et",
      "incididunt",
      "reprehenderit",
      "dolor"
    ],
    "friends": [
      {
        "id": 0,
        "name": "Blanche Mccarty"
      },
      {
        "id": 1,
        "name": "Mccray Walter"
      },
      {
        "id": 2,
        "name": "Georgina Anderson"
      }
    ],
    "greeting": "Hello, Priscilla Albert! You have 9 unread messages.",
    "favoriteFruit": "strawberry"
  },
  {
    "_id": "64b13dbcccbf4ff49ea8e5ab",
    "index": 5,
    "guid": "4d33c9b6-7e78-42dc-b53d-70c03bd357b1",
    "isActive": true,
    "balance": "$3,830.25",
    "picture": "http://placehold.it/32x32",
    "age": 23,
    "eyeColor": "brown",
    "name": "Darla Rodriguez",
    "gender": "female",
    "company": "QIMONK",
    "email": "darlarodriguez@qimonk.com",
    "phone": "+1 (898) 482-2944",
    "address": "146 Eldert Lane, Fairview, Arkansas, 6958",
    "about": "Officia pariatur aliqua dolore nisi anim occaecat excepteur in esse qui nisi. Dolore anim proident fugiat eiusmod dolor duis ea Lorem consectetur officia Lorem laboris non exercitation. Anim eu amet sit aute amet id nostrud fugiat excepteur velit. Magna dolor aliquip consequat eiusmod aute est sunt duis. Irure amet nostrud minim cillum qui laborum nostrud est veniam nisi sunt Lorem velit. Exercitation irure dolor magna sit voluptate aute enim enim consectetur sit velit proident anim ad. Sint et officia ipsum duis cillum mollit enim aliquip eu.\r\n",
    "registered": "2017-04-07T01:53:29 -02:00",
    "latitude": -60.503285,
    "longitude": -161.891942,
    "tags": [
      "ullamco",
      "fugiat",
      "fugiat",
      "reprehenderit",
      "non",
      "ea",
      "voluptate"
    ],
    "friends": [
      {
        "id": 0,
        "name": "Rosalie Cummings"
      },
      {
        "id": 1,
        "name": "Stark Carroll"
      },
      {
        "id": 2,
        "name": "Alyson Sharpe"
      }
    ],
    "greeting": "Hello, Darla Rodriguez! You have 6 unread messages.",
    "favoriteFruit": "banana"
  },
  {
    "_id": "64b13dbc6e2cf83674248e18",
    "index": 6,
    "guid": "95ee8391-4045-4201-8ecd-80180e597cb6",
    "isActive": false,
    "balance": "$1,394.69",
    "picture": "http://placehold.it/32x32",
    "age": 29,
    "eyeColor": "brown",
    "name": "Reba Singleton",
    "gender": "female",
    "company": "DOGNOSIS",
    "email": "rebasingleton@dognosis.com",
    "phone": "+1 (832) 411-2158",
    "address": "340 Dorset Street, Roy, Delaware, 2885",
    "about": "Ut occaecat cillum sint esse deserunt elit cillum qui quis exercitation in. Velit nisi ipsum quis sint mollit ea exercitation cupidatat non nisi ut. Nostrud duis esse dolor Lorem. Nisi tempor aliqua pariatur officia. Commodo nulla eiusmod cupidatat aliquip in mollit. Anim ex nulla do sit. Esse exercitation esse Lorem sit consectetur qui exercitation qui ipsum deserunt.\r\n",
    "registered": "2018-01-31T12:29:27 -01:00",
    "latitude": -2.43137,
    "longitude": 133.655031,
    "tags": [
      "nisi",
      "in",
      "exercitation",
      "enim",
      "exercitation",
      "magna",
      "ea"
    ],
    "friends": [
      {
        "id": 0,
        "name": "Ratliff Tucker"
      },
      {
        "id": 1,
        "name": "Shana Copeland"
      },
      {
        "id": 2,
        "name": "Levy Barlow"
      }
    ],
    "greeting": "Hello, Reba Singleton! You have 9 unread messages.",
    "favoriteFruit": "banana"
  }
]
"#;

build_tree! {
    JSON => #s*, @JValue, #s*;

    Number #=> "-"?, ("0" | [1-9], [0-9]*), (".", [0-9]+, EPart?)?;
    EPart #=> ("E" | "e"), ("-" | "+")?, #d+;

    JString => "\"",
                    @StringFragment*
              ,"\"";

    StringFragment => @[^"\""]+ <NormalChars>
                   |  @Escape <EscapedChar>;

    Escape => "\\", @EscapeType;

    EscapeType =>"n" <Newline>
               | "\\" <Backslash>
               | "\"" <DQuote>
               | "r" <Return>
               | "t" <Tab>
               | "f" <FormFeed>
               | "/" <ForwardSlash>
               | "u", @Hex, @Hex, @Hex, @Hex <Unicode>;

    JValue  =>  @JString
            | @Number
            | @Object
            | @Array
            | "true" <True>
            | "false" <False>
            | "null" <Null>;

    Object => "{", #s*, @ObjectElems?, #s*, "}";
    ObjectKV => @JString, #s*, ":", #s*, @JValue, #s*;
    ObjectElems => @ObjectKV, @(",", #s*, @ObjectKV)*;

    Array => "[", #s*, @ArrayElems?, #s*, "]";
    ArrayElems => @JValue, #s*, @(",", #s*, @JValue, #s*)*;

    Hex #=> [0-9a-fA-F];
}

impl TryInto<HashMap<String, JValue>> for Object {
    type Error = ();
    fn try_into(self) -> Result<HashMap<String, JValue>, Self::Error> {
        let mut hash_map: HashMap<String, JValue> = HashMap::new();
        let elems = match self.0 {
            Some(a) => *a,
            None => return Ok(HashMap::new()),
        };
        let ObjectElems(first, rest) = elems;
        let ObjectKV(k, v) = *first;
        hash_map.insert(k.to_string(), *v).unwrap();
        for item in rest {
            let ObjectKV(k, v) = *item;
            hash_map.insert(k.to_string(), *v).ok_or(())?;
        }
        Ok(hash_map)
    }
}

impl From<Array> for Vec<Box<JValue>> {
    fn from(arr: Array) -> Vec<Box<JValue>> {
        let elems = match arr.0 {
            Some(a) => *a,
            None => return vec![],
        };
        let ArrayElems(first, mut rest) = elems;
        rest.insert(0, first);
        rest
    }
}

impl Into<f64> for Number {
    fn into(self) -> f64 {
        self.0.parse().unwrap()
    }
}

impl std::string::ToString for JString {
    fn to_string(&self) -> String {
        self.0
            .iter()
            .fold("".to_owned(), |x, y| match (*y).as_ref() {
                StringFragment::NormalChars(str) => x + &str,
                StringFragment::EscapedChar(esc) => {
                    let Escape(esc) = esc.as_ref();
                    match esc.as_ref() {
                        EscapeType::Tab() => x + "\t",
                        EscapeType::DQuote() => x + "\"",
                        EscapeType::Return() => x + "\r",
                        EscapeType::Newline() => x + "\n",
                        EscapeType::Backslash() => x + "\\",
                        EscapeType::FormFeed() => x + "\u{000C}",
                        EscapeType::ForwardSlash() => x + "/",
                        EscapeType::Unicode(a, b, c, d) => {
                            let (Hex(a), Hex(b), Hex(c), Hex(d)) =
                                (a.as_ref(), b.as_ref(), c.as_ref(), d.as_ref());
                            let val = format!("{}{}{}{}", a, b, c, d);
                            let val = u32::from_str_radix(&val, 16)
                                .ok()
                                .and_then(std::char::from_u32)
                                .unwrap()
                                .to_string();
                            x + &val
                        }
                    }
                }
            })
    }
}

fn main() {
    let parsed = Array::parse(
        r#"[ true, -1888300249.8418272, 1140199957, -1319010735, "success", -1523848246.6416917 ]"#,
    );
    let _parsed = match parsed {
        Ok((_, out)) => out,
        Err(nom::Err::Error(e)) => panic!(
            "{}",
            nom::error::convert_error(
                r#"[ true, -1888300249.8418272, 1140199957, -1319010735, "success", -1523848246.6416917 ]"#,
                e
            )
        ),
        _ => panic!("??"),
    };
    // println!("{:#?}", parsed);

    // let input = std::fs::read_to_string("../../../examples/test.json").unwrap();
    //
    //    // let t = std::time::Instant::now();
    // let t = std::time::Instant::now();
    let parsed = JSON::parse(TEST1);
    // println!("Time to parse: {:?}", t.elapsed());
    let _parsed: Vec<Box<JValue>> = match parsed {
        Ok((_, out)) => match *out {
            JSON(val) => match *val {
                JValue::Array(arr) => (*arr).into(),
                _ => panic!("Unexpected JSON Datatype parsed"),
            },
        },
        Err(nom::Err::Error(e)) => panic!("{}", nom::error::convert_error(
                TEST1,
                e
            )),
        _ => panic!("Some error in the source code"),
    };
    //    println!("{:#?}", parsed);
}
