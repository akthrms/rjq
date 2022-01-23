use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::IResult;
use nom::sequence::{delimited, tuple};

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
    let (input, (digit, filter)) =
        tuple((delimited(tag("["), digit1, tag("]")), parse_filter))(input)?;
    let filter = Filter::Index(digit.parse().unwrap(), Box::new(filter));
    Ok((input, filter))
}

fn parse_null(input: &str) -> IResult<&str, Filter> {
    let filter = Filter::Null;
    Ok((input, filter))
}
