use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, digit1};
use nom::combinator::{eof, recognize};
use nom::multi::many1;
use nom::sequence::{delimited, tuple};
use nom::IResult;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
enum Filter {
    Field(String, Box<Filter>),
    Index(usize, Box<Filter>),
    Null,
}

fn parse_filter(input: &str) -> IResult<&str, Filter> {
    let parse_filters = alt((parse_field, parse_index, parse_null));
    let (input, (_, filter, _)) = tuple((tag("."), parse_filters, eof))(input)?;
    Ok((input, filter))
}

fn parse_filter_rec(input: &str) -> IResult<&str, Filter> {
    fn parse_field_with_dot(input: &str) -> IResult<&str, Filter> {
        let (input, (_, filter)) = tuple((tag("."), parse_field))(input)?;
        Ok((input, filter))
    }

    let (input, filter) = alt((parse_field_with_dot, parse_index, parse_null))(input)?;
    Ok((input, filter))
}

fn parse_field(input: &str) -> IResult<&str, Filter> {
    let parse_word = recognize(many1(alt((alphanumeric1, tag("-"), tag("_")))));
    let (input, (word, filter)) = tuple((parse_word, parse_filter_rec))(input)?;
    let filter = Filter::Field(word.to_string(), Box::new(filter));
    Ok((input, filter))
}

fn parse_index(input: &str) -> IResult<&str, Filter> {
    let parse_digit = delimited(tag("["), digit1, tag("]"));
    let (input, (digit, filter)) = tuple((parse_digit, parse_filter_rec))(input)?;
    let filter = Filter::Index(digit.parse().unwrap(), Box::new(filter));
    Ok((input, filter))
}

fn parse_null(input: &str) -> IResult<&str, Filter> {
    let filter = Filter::Null;
    Ok((input, filter))
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
enum Query {
    Object(Vec<(String, Query)>),
    Array(Vec<Query>),
    Filter(Filter),
}

#[cfg(test)]
mod tests {
    use crate::parser::{parse_filter, Filter};

    #[test]
    fn test_parse1() {
        assert_eq!(parse_filter("."), Ok(("", Filter::Null)));
    }

    #[test]
    fn test_parse2() {
        assert_eq!(
            parse_filter(".[0]"),
            Ok(("", Filter::Index(0, Box::new(Filter::Null))))
        );
    }

    #[test]
    fn test_parse3() {
        assert_eq!(
            parse_filter(".fieldName"),
            Ok((
                "",
                Filter::Field("fieldName".to_string(), Box::new(Filter::Null))
            ))
        );
    }

    #[test]
    fn test_parse4() {
        assert_eq!(
            parse_filter(".[0].fieldName"),
            Ok((
                "",
                Filter::Index(
                    0,
                    Box::new(Filter::Field(
                        "fieldName".to_string(),
                        Box::new(Filter::Null)
                    ))
                )
            ))
        );
    }

    #[test]
    fn test_parse5() {
        assert_eq!(
            parse_filter(".fieldName[0]"),
            Ok((
                "",
                Filter::Field(
                    "fieldName".to_string(),
                    Box::new(Filter::Index(0, Box::new(Filter::Null)))
                )
            ))
        );
    }
}
