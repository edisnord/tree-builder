use core::ops::RangeFrom;
use nom::error::ParseError;
use nom::IResult;
use nom::{AsChar, InputIter, InputTake, Slice};

pub fn satisfy_one<F, I, Error: ParseError<I>>(cond: F) -> impl Fn(I) -> IResult<I, I, Error>
where
    I: Slice<RangeFrom<usize>> + InputIter + InputTake,
    <I as InputIter>::Item: AsChar,
    F: Fn(char) -> bool,
{
    move |input: I| {
        if input.iter_elements().next().is_none() {
            return Err(nom::Err::Error(Error::from_error_kind(
                input,
                nom::error::ErrorKind::Eof,
            )));
        }
        match input.take(1usize).iter_elements().nth(0).map(|a| {
            if cond(a.as_char()) {
                (input.take(1), true)
            } else {
                (input.take(1), false)
            }
        }) {
            Some((slice, matched)) => {
                if matched {
                    Ok((input.slice(1usize..), slice))
                } else {
                    Err(nom::Err::Error(Error::from_error_kind(
                        input,
                        nom::error::ErrorKind::Satisfy,
                    )))
                }
            }
            _ => Err(nom::Err::Error(Error::from_error_kind(
                input,
                nom::error::ErrorKind::Eof,
            ))),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::public_parsers::satisfy_one;

    #[test]
    fn satisfy_one_test() {
        let input = "";
        match satisfy_one::<_, _, (&str, nom::error::ErrorKind)>(|_| true)(input) {
            Ok(_) => panic!("Wrong behavior, should return error with ErrorKind EOF"),
            Err(nom::Err::Error((_, code))) => match code {
                nom::error::ErrorKind::Eof => (),
                _ => panic!("Wrong behavior, should return error with ErrorKind EOF"),
            },
            Err(_) => panic!("?? What are these error types"),
        }
    }
}
