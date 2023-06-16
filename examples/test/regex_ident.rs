use tree_builder_macro::rule;

rule! { Test #=> #s }

rule! { Whatever #=> "Bruh", #d+, Test+}
rule! { Whatever2 #=> "Bruh", #d+, Test*}
rule! {Test2 => Test?}
rule! {Test3 => Test, Test2}
fn main(){}
