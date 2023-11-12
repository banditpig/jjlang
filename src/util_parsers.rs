use nom::{
    IResult,
    error::ParseError,
    sequence::delimited,
    character::complete::multispace0,
    bytes::complete::{tag,take_till}
};
pub fn comment<'i, E>(i: &'i str) -> IResult<&'i str, &'i str, E>
    where
        E: ParseError<&'i str>
{
    let (i, _) = tag("//")(i)?;
    let (i, content) = take_till(|ch| ch == '\n')(i)?;
    if content.starts_with(' ') {
        Ok((i, &content[1..]))
    } else {
        Ok((i, content))
    }
}

pub(crate) fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
    where
        F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(
        multispace0,
        inner,
        multispace0
    )
}