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
    unimplemented!();
}

fn parse_null(input: &str) -> IResult<&str, Filter> {
    let filter = Filter::Null;
    Ok((input, filter))
}
