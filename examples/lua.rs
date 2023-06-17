use tree_builder::build_tree;

//WIP
//build_tree!{
//    Expression => "nil" <Nil>
//               | "false" <False>
//               | "true" <True>
//               | @Number
//               | @LString
//               | "..." <DotDotDot>
//               | @Function
//               | @PrefixExpression
//               | @TableConstructor
//               // Mund te jete left-recursive
//               | @Expression, @BinaryOperator, @Expression <BinaryApplication>
//               // Mund te jete left-recursive
//               | @PreUnaryOperator, @Expression <PreUnaryApplication>
//               | @Expression, @PostUnop <PostUnaryOpApplication>
//               | @Expression, "is", @Type <TypeCheck>;
//
//    PrefixExpression => @Variable
//                     | "(", @Expression, ")" <Expr>
//                     // Mund te jete left-recursive
//                     | @FunctionCall;
//
//    FunctionCall => @PrefixExpression, @Arguments <StdCall>
//                 | @PrefixExpression, ":", @Name, @Arguments <IdkCall>;
//
//    Argument => @(@Name, ":")?, @Expression;
//
//    Arguments => "(", @ArgumentList?, ")" <ArgList>
//              | @TableConstructor
//              | @LString;
//
//    Function => "function", @FunctionBody;
//
//    ReturnType => ":", @Type, @(",", @Type)*;
//
//    Parameter => @Name?, @TypedName, @("=", @Expression);
//
//    ParameterList => @Parameter, @(",", @Paramer)*, ParamListEnd?;
//
//    ParamListEnd => ","?, "..." <Varargs>
//                 | "," <OptionalComma>;
//
//    FunctionHead => "(", @ParameterList?, ")", @ReturnType;
//
//    FunctionBody => @FunctionHead, @Block, "end";
//
//    ClassBody => @ClassMember*, "end";
//
//    ClassMember => @Property
//                |  @Default
//                |  @Function;
//
//    Property => "property", @TypedName, @("=", @Expression);
//
//    Default => "default", @Name, "=", @Expression;
//
//    Enum => "enum", @EnumBody;
//
//    EnumCase => @Name, @("(", @(@Name, ":")?, @Type, ")")?, @("=", @Expression)?;
//
//    EnumBody => @EnumCase, (",", @EnumCase)*, "end";
//
//    Protocol => "protocol", @Name, @(":", @Name, @(",", @Name)*)?, @ProtocolBody;
//
//    ProtocolBody => @ProtocolType*, "end";
//    ProtocolType => "property", @TypedName <Property>
//                 | "function", @FunctionHead <Function>;
//
//    TableConstructor => "{", @FieldList?, "}";
//
//    FieldList => @Field, @(FieldSep, @Field)*, FieldSep?;
//
//    Field => "[", @Expression ,"]", "=", @Expression <KeyToVal>
//        | @Name, "=", @Expression  <Assign>
//        | @Expression ;
//
//    FieldSep #=> "," | ";";
//
//    Binop => "+" <Plus>
//          |  "-" <Minus>
//          |  "*" <Star>
//          |  "/" <Slash>
//          |  "^" <Caret>
//          |  "%" <Perc>
//          |  ".." <DotDot>
//          |  "<" <Less>
//          |  "<=" <LessEq>
//          |  ">" <Greater>
//          |  ">=" <GreaterEq>
//          |  "==" <Eq>
//          |  "~=" <NEq>
//          |  "and" <And>
//          |  "or" <Or>;
//
//    PreUnop => "-" <Minus>
//        |   "#" <Hash>
//        |   "not" <Not>;
//
//    PostUnop => "++" <Increment>
//            |   "--" <Decrement>
//}

fn main(){}
