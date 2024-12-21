use nom::{
  character::complete::{multispace0, space0},
  error::ParseError,
  sequence::delimited,
  IResult,
};

pub fn ws_line<'a, F: 'a, O, E: ParseError<&'a str>>(
  inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
  F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
  delimited(space0, inner, space0)
}

pub fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
  inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
  F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
  delimited(multispace0, inner, multispace0)
}
