use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::sequence::{delimited, tuple};
use nom::IResult;

enum Filter {
    Field(String, Box<Filter>),
    Index(isize, Box<Filter>),
    Null,
}

fn parse_filter(input: &str) -> IResult<&str, Filter> {
    unimplemented!();
}

fn parse_field(input: &str) -> IResult<&str, Filter> {
    unimplemented!();
}

fn parse_index(input: &str) -> IResult<&str, Filter> {
    let parse_digit = delimited(tag("["), digit1, tag("]"));
    let (input, (digit, filter)) = tuple((parse_digit, parse_filter))(input)?;
    let filter = Filter::Index(digit.parse().unwrap(), Box::new(filter));
    Ok((input, filter))
}

fn parse_null(input: &str) -> IResult<&str, Filter> {
    let filter = Filter::Null;
    Ok((input, filter))
}
