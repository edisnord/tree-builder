use nom::IResult;
use nom::error::{context, ParseError};
use nom::{InputTake, Slice, InputIter, AsChar};
use core::ops::RangeFrom;


pub fn satisfy_one<F, I, Error: ParseError<I>>(cond: F) -> impl Fn(I) -> IResult<I, I, Error>
    where
        I: Slice<RangeFrom<usize>> + InputIter + InputTake,
        <I as InputIter>::Item: AsChar,
        F: Fn(char) -> bool,
{
    move |input: I| {
        if input.iter_elements().next().is_none() {
            return Err(nom::Err::Error(Error::from_error_kind(input, nom::error::ErrorKind::Eof)));
        }
        match input.take(1usize).iter_elements().nth(0).map(|a| {
            if cond(a.as_char()) {
                (input.take(1), true)
            } else {
                (input.take(1), false)
            }
        }) {
            Some((slice, matched)) => if matched {
                Ok((input.slice(1usize..), slice))
            } else {
                Err(nom::Err::Error(Error::from_error_kind(input, nom::error::ErrorKind::Satisfy)))
            },
            _ => Err(nom::Err::Error(Error::from_error_kind(input, nom::error::ErrorKind::Eof)))
        }
    }
}


pub fn digit<I: Clone, Error: ParseError<I> + nom::error::ContextError<I>>() -> impl Fn(I) -> IResult<I, I, Error>
    where
        I: Slice<RangeFrom<usize>> + InputIter + InputTake,
        <I as InputIter>::Item: AsChar,
{
    |input| context("\\d", satisfy_one(|x| x.is_digit(10)))(input)
}

pub fn non_digit<I: Clone, Error: ParseError<I> + nom::error::ContextError<I>>() -> impl Fn(I) -> IResult<I, I, Error>
    where
        I: Slice<RangeFrom<usize>> + InputIter + InputTake,
        <I as InputIter>::Item: AsChar,
{
    |input| context("\\D", satisfy_one(|x| !x.is_digit(10)))(input)
}

pub fn alpha_num_underscore<I: Clone, Error: ParseError<I> + nom::error::ContextError<I>>() -> impl Fn(I) -> IResult<I, I, Error>
    where
        I: Slice<RangeFrom<usize>> + InputIter + InputTake,
        <I as InputIter>::Item: AsChar,
{
    |input| context("\\w", satisfy_one(|x| x.is_alpha() || x.is_numeric() || x == '_' ))(input)
}

pub fn non_alpha_num_underscore<I: Clone, Error: ParseError<I> + nom::error::ContextError<I>>() -> impl Fn(I) -> IResult<I, I, Error>
    where
        I: Slice<RangeFrom<usize>> + InputIter + InputTake,
        <I as InputIter>::Item: AsChar,
{
    |input| context("\\W", satisfy_one(|x| !(x.is_alpha() || x.is_numeric() || x == '_')))(input)
}

pub fn whitespace<I: Clone, Error: ParseError<I> + nom::error::ContextError<I>>() -> impl Fn(I) -> IResult<I, I, Error>
    where
        I: Slice<RangeFrom<usize>> + InputIter + InputTake,
        <I as InputIter>::Item: AsChar,
{
    |input| context("\\s", satisfy_one(|x| x.is_whitespace()))(input)
}

pub fn not_whitespace<I: Clone, Error: ParseError<I> + nom::error::ContextError<I>>() -> impl Fn(I) -> IResult<I, I, Error>
    where
        I: Slice<RangeFrom<usize>> + InputIter + InputTake,
        <I as InputIter>::Item: AsChar,
{
    |input| context("\\S", satisfy_one(|x| !x.is_whitespace()))(input)
}

pub fn any_char<I: Clone, Error: ParseError<I> + nom::error::ContextError<I>>() -> impl Fn(I) -> IResult<I, I, Error>
    where
        I: Slice<RangeFrom<usize>> + InputIter + InputTake,
        <I as InputIter>::Item: AsChar,
{
    |input| context(".", satisfy_one(|_|true))(input)
}

#[cfg(test)]
mod test {
    use nom::Finish;
    use crate::public_parsers::{digit, non_digit, satisfy_one, alpha_num_underscore, non_alpha_num_underscore};

    #[test]
    fn satisfy_one_test() {
        let input ="";
        match satisfy_one::<_, _, (&str, nom::error::ErrorKind)>(|_| true)(input) {
            Ok(_) => panic!("Wrong behavior, should return error with ErrorKind EOF"),
            Err(nom::Err::Error((_, code))) => match code {
                nom::error::ErrorKind::Eof => (),
                _ => panic!("Wrong behavior, should return error with ErrorKind EOF")
            },
            Err(_) => panic!("?? What are these error types")
        }
    }

    #[test]
    fn digit_test() {
        let input = "123";
        let (rem, res)= digit::<_, (&str, nom::error::ErrorKind)>()(input).unwrap();
        assert_eq!(rem, "23");
        assert_eq!(res, "1");
    }

    #[test]
    fn non_digit_test() {
        let input = "a23";
    let (rem, res): (&str, &str) = non_digit::<_, nom::error::VerboseError<&str>>()(input).unwrap();
    assert_eq!(rem, "23");
    assert_eq!(res, "a");
    let input = "123";
    let e = non_digit::<_, nom::error::VerboseError<&str>>()(input).finish().err().unwrap();
}

#[test]
fn alphanumunder_test() {
    let input = "a23";
    let (rem, res): (&str, &str) = alpha_num_underscore::<_, nom::error::VerboseError<&str>>()(input).unwrap();
    assert_eq!(rem, "23");
    assert_eq!(res, "a");
    let input = "123";
    let (rem, res): (&str, &str) = alpha_num_underscore::<_, nom::error::VerboseError<&str>>()(input).unwrap();
    assert_eq!(rem, "23");
        assert_eq!(res, "1");
        let input = "_23";
        let (rem, res): (&str, &str) = alpha_num_underscore::<_, nom::error::VerboseError<&str>>()(input).unwrap();
        assert_eq!(rem, "23");
        assert_eq!(res, "_");
        let input = " 123";
        let e = alpha_num_underscore::<_, nom::error::VerboseError<&str>>()(input).finish().err().unwrap();
    }

    #[test]
    fn not_alphanumunder_test() {
        let input = "!23";
        let (rem, res): (&str, &str) = non_alpha_num_underscore::<_, nom::error::VerboseError<&str>>()(input).unwrap();
        assert_eq!(rem, "23");
        assert_eq!(res, "!");
        let input = " 23";
        let (rem, res): (&str, &str) = non_alpha_num_underscore::<_, nom::error::VerboseError<&str>>()(input).unwrap();
        assert_eq!(rem, "23");
        assert_eq!(res, " ");
        let input = "?23";
        let (rem, res): (&str, &str) = non_alpha_num_underscore::<_, nom::error::VerboseError<&str>>()(input).unwrap();
        assert_eq!(rem, "23");
        assert_eq!(res, "?");
        let input = "_123";
        let e = non_alpha_num_underscore::<_, nom::error::VerboseError<&str>>()(input).finish().err().unwrap();
    }
}
