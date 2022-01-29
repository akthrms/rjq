use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, digit1};
use nom::combinator::{eof, recognize};
use nom::multi::{many1, separated_list0};
use nom::sequence::{delimited, tuple};
use nom::IResult;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Filter {
    Field(String, Box<Filter>),
    Index(usize, Box<Filter>),
    Null,
}

pub fn parse_filter(input: &str) -> IResult<&str, Filter> {
    let (input, (_, filter, _)) = tuple((tag("."), choice_filter, eof))(input)?;
    Ok((input, filter))
}

fn choice_filter(input: &str) -> IResult<&str, Filter> {
    let (input, filter) = alt((parse_f_field, parse_f_index, parse_f_null))(input)?;
    Ok((input, filter))
}

fn parse_f_rec(input: &str) -> IResult<&str, Filter> {
    fn parse_f_field_with_dot(input: &str) -> IResult<&str, Filter> {
        let (input, (_, filter)) = tuple((tag("."), parse_f_field))(input)?;
        Ok((input, filter))
    }

    let (input, filter) = alt((parse_f_field_with_dot, parse_f_index, parse_f_null))(input)?;
    Ok((input, filter))
}

fn parse_f_field(input: &str) -> IResult<&str, Filter> {
    let parse_word = recognize(many1(alt((alphanumeric1, tag("-"), tag("_")))));
    let (input, (word, filter)) = tuple((parse_word, parse_f_rec))(input)?;
    let filter = Filter::Field(word.to_string(), Box::new(filter));
    Ok((input, filter))
}

fn parse_f_index(input: &str) -> IResult<&str, Filter> {
    let parse_digit = delimited(tag("["), digit1, tag("]"));
    let (input, (digit, filter)) = tuple((parse_digit, parse_f_rec))(input)?;
    let filter = Filter::Index(digit.parse().unwrap(), Box::new(filter));
    Ok((input, filter))
}

fn parse_f_null(input: &str) -> IResult<&str, Filter> {
    let filter = Filter::Null;
    Ok((input, filter))
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Query {
    Object(Vec<(String, Query)>),
    Array(Vec<Query>),
    Filter(Filter),
}

pub fn parse_query(input: &str) -> IResult<&str, Query> {
    let (input, (query, _)) = tuple((choice_query, eof))(input)?;
    Ok((input, query))
}

fn choice_query(input: &str) -> IResult<&str, Query> {
    let (input, query) = alt((parse_q_object, parse_q_array, parse_q_filter))(input)?;
    Ok((input, query))
}

fn parse_q_object(input: &str) -> IResult<&str, Query> {
    fn parse_key_value(input: &str) -> IResult<&str, (String, Query)> {
        fn parse_key(input: &str) -> IResult<&str, &str> {
            let parse_word = recognize(many1(alt((alphanumeric1, tag("-"), tag("_")))));
            let (input, key) = delimited(tag("\""), parse_word, tag("\""))(input)?;
            Ok((input, key))
        }

        fn parse_value(input: &str) -> IResult<&str, Query> {
            let (input, (_, value)) = tuple((tag(":"), choice_query))(input)?;
            Ok((input, value))
        }

        let (input, (key, value)) = tuple((parse_key, parse_value))(input)?;
        Ok((input, (key.to_string(), value)))
    }

    let parse_object = separated_list0(tag(","), parse_key_value);
    let (input, object) = delimited(tag("{"), parse_object, tag("}"))(input)?;
    let query = Query::Object(object);
    Ok((input, query))
}

fn parse_q_array(input: &str) -> IResult<&str, Query> {
    let parse_array = separated_list0(tag(","), choice_query);
    let (input, queries) = delimited(tag("["), parse_array, tag("]"))(input)?;
    let query = Query::Array(queries);
    Ok((input, query))
}

fn parse_q_filter(input: &str) -> IResult<&str, Query> {
    fn parse_filter(input: &str) -> IResult<&str, Filter> {
        let (input, (_, filter)) = tuple((tag("."), choice_filter))(input)?;
        Ok((input, filter))
    }

    let (input, filter) = parse_filter(input)?;
    let query = Query::Filter(filter);
    Ok((input, query))
}

#[cfg(test)]
mod tests {
    use crate::parser::{parse_filter, parse_query, Filter, Query};

    #[test]
    fn test_parse_filter1() {
        assert_eq!(parse_filter("."), Ok(("", Filter::Null)));
    }

    #[test]
    fn test_parse_filter2() {
        assert_eq!(
            parse_filter(".[0]"),
            Ok(("", Filter::Index(0, Box::new(Filter::Null))))
        );
    }

    #[test]
    fn test_parse_filter3() {
        assert_eq!(
            parse_filter(".hoge"),
            Ok((
                "",
                Filter::Field("hoge".to_string(), Box::new(Filter::Null))
            ))
        );
    }

    #[test]
    fn test_parse_filter4() {
        assert_eq!(
            parse_filter(".[0].hoge"),
            Ok((
                "",
                Filter::Index(
                    0,
                    Box::new(Filter::Field("hoge".to_string(), Box::new(Filter::Null)))
                )
            ))
        );
    }

    #[test]
    fn test_parse_filter5() {
        assert_eq!(
            parse_filter(".hoge[0]"),
            Ok((
                "",
                Filter::Field(
                    "hoge".to_string(),
                    Box::new(Filter::Index(0, Box::new(Filter::Null)))
                )
            ))
        );
    }

    #[test]
    fn test_parse_query1() {
        assert_eq!(parse_query("[]"), Ok(("", Query::Array(vec![]))));
    }

    #[test]
    fn test_parse_query2() {
        assert_eq!(
            parse_query("[.hoge,.piyo]"),
            Ok((
                "",
                Query::Array(vec![
                    Query::Filter(Filter::Field("hoge".to_string(), Box::new(Filter::Null))),
                    Query::Filter(Filter::Field("piyo".to_string(), Box::new(Filter::Null)))
                ])
            ))
        );
    }

    #[test]
    fn test_parse_query3() {
        assert_eq!(
            parse_query("{\"hoge\":[],\"piyo\":[]}"),
            Ok((
                "",
                Query::Object(vec![
                    ("hoge".to_string(), Query::Array(vec![])),
                    ("piyo".to_string(), Query::Array(vec![]))
                ])
            ))
        );
    }
}
