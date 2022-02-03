use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, digit1, multispace0};
use nom::combinator::eof;
use nom::multi::{many1, separated_list0};
use nom::sequence::{delimited, tuple};
use nom::IResult;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Filter {
    Field(String, Box<Filter>),
    Index(usize, Box<Filter>),
    Null,
}

pub fn parse_filter(input: &str) -> Result<Filter, String> {
    tuple((parse_tag("."), choice_filter, eof))(input)
        .map(|(_, (_, filter, _))| filter)
        .map_err(|_| format!("invalid filter format: {}", input)) // TODO: detail error message
}

fn choice_filter(input: &str) -> IResult<&str, Filter> {
    let (input, filter) = alt((parse_f_field, parse_f_index, parse_f_null))(input)?;
    Ok((input, filter))
}

fn parse_f_rec(input: &str) -> IResult<&str, Filter> {
    fn parse_f_field_with_dot(input: &str) -> IResult<&str, Filter> {
        let (input, (_, filter)) = tuple((parse_tag("."), parse_f_field))(input)?;
        Ok((input, filter))
    }

    let (input, filter) = alt((parse_f_field_with_dot, parse_f_index, parse_f_null))(input)?;
    Ok((input, filter))
}

fn parse_f_field(input: &str) -> IResult<&str, Filter> {
    let (input, (word, filter)) = tuple((parse_word, parse_f_rec))(input)?;
    let filter = Filter::Field(word, Box::new(filter));
    Ok((input, filter))
}

fn parse_f_index(input: &str) -> IResult<&str, Filter> {
    let parse_digit = delimited(parse_tag("["), digit1, parse_tag("]"));
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

pub fn parse_query(input: &str) -> Result<Query, String> {
    tuple((choice_query, eof))(input)
        .map(|(_, (filter, _))| filter)
        .map_err(|_| format!("invalid query format: {}", input)) // TODO: detail error message
}

fn choice_query(input: &str) -> IResult<&str, Query> {
    let (input, query) = alt((parse_q_object, parse_q_array, parse_q_filter))(input)?;
    Ok((input, query))
}

fn parse_q_object(input: &str) -> IResult<&str, Query> {
    fn parse_key_value(input: &str) -> IResult<&str, (String, Query)> {
        fn parse_key(input: &str) -> IResult<&str, String> {
            let (input, key) = delimited(parse_tag("\""), parse_word, parse_tag("\""))(input)?;
            Ok((input, key))
        }

        fn parse_value(input: &str) -> IResult<&str, Query> {
            let (input, (_, value)) = tuple((parse_tag(":"), choice_query))(input)?;
            Ok((input, value))
        }

        let (input, (key, value)) = tuple((parse_key, parse_value))(input)?;
        Ok((input, (key, value)))
    }

    let parse_object = separated_list0(parse_tag(","), parse_key_value);
    let (input, object) = delimited(parse_tag("{"), parse_object, parse_tag("}"))(input)?;
    let query = Query::Object(object);
    Ok((input, query))
}

fn parse_q_array(input: &str) -> IResult<&str, Query> {
    let parse_array = separated_list0(parse_tag(","), choice_query);
    let (input, queries) = delimited(parse_tag("["), parse_array, parse_tag("]"))(input)?;
    let query = Query::Array(queries);
    Ok((input, query))
}

fn parse_q_filter(input: &str) -> IResult<&str, Query> {
    fn parse_filter(input: &str) -> IResult<&str, Filter> {
        let (input, (_, filter)) = tuple((parse_tag("."), choice_filter))(input)?;
        Ok((input, filter))
    }

    let (input, filter) = parse_filter(input)?;
    let query = Query::Filter(filter);
    Ok((input, query))
}

fn parse_tag<'a>(input: &'a str) -> impl FnMut(&'a str) -> IResult<&'a str, &str> {
    delimited(multispace0, tag(input), multispace0)
}

fn parse_word(input: &str) -> IResult<&str, String> {
    let (input, words) = delimited(
        multispace0,
        many1(alt((alphanumeric1, tag("-"), tag("_")))),
        multispace0,
    )(input)?;
    Ok((input, words.join("")))
}

#[cfg(test)]
mod tests {
    use crate::parser::{parse_filter, parse_query, Filter, Query};

    #[test]
    fn test_parse_filter1() {
        assert_eq!(parse_filter("."), Ok(Filter::Null));
    }

    #[test]
    fn test_parse_filter2() {
        assert_eq!(
            parse_filter(".[0]"),
            Ok(Filter::Index(0, Box::new(Filter::Null)))
        );
    }

    #[test]
    fn test_parse_filter3() {
        assert_eq!(
            parse_filter(".hoge"),
            Ok(Filter::Field("hoge".to_string(), Box::new(Filter::Null)))
        );
    }

    #[test]
    fn test_parse_filter4() {
        assert_eq!(
            parse_filter(".[0].hoge"),
            Ok(Filter::Index(
                0,
                Box::new(Filter::Field("hoge".to_string(), Box::new(Filter::Null)))
            ))
        );
    }

    #[test]
    fn test_parse_filter5() {
        assert_eq!(
            parse_filter(".hoge[0]"),
            Ok(Filter::Field(
                "hoge".to_string(),
                Box::new(Filter::Index(0, Box::new(Filter::Null)))
            ))
        );
    }

    #[test]
    fn test_parse_filter_spaces1() {
        assert_eq!(parse_filter(" . "), Ok(Filter::Null));
    }

    #[test]
    fn test_parse_filter_spaces2() {
        assert_eq!(
            parse_filter(" . [ 0 ] "),
            Ok(Filter::Index(0, Box::new(Filter::Null)))
        );
    }

    #[test]
    fn test_parse_filter_spaces3() {
        assert_eq!(
            parse_filter(" . hoge "),
            Ok(Filter::Field("hoge".to_string(), Box::new(Filter::Null)))
        );
    }

    #[test]
    fn test_parse_filter_spaces4() {
        assert_eq!(
            parse_filter(" . [ 0 ] . hoge "),
            Ok(Filter::Index(
                0,
                Box::new(Filter::Field("hoge".to_string(), Box::new(Filter::Null)))
            ))
        );
    }

    #[test]
    fn test_parse_filter_spaces5() {
        assert_eq!(
            parse_filter(" . hoge [ 0 ] "),
            Ok(Filter::Field(
                "hoge".to_string(),
                Box::new(Filter::Index(0, Box::new(Filter::Null)))
            ))
        );
    }

    #[test]
    fn test_parse_query1() {
        assert_eq!(parse_query("[]"), Ok(Query::Array(vec![])));
    }

    #[test]
    fn test_parse_query2() {
        assert_eq!(
            parse_query("[.hoge,.piyo]"),
            Ok(Query::Array(vec![
                Query::Filter(Filter::Field("hoge".to_string(), Box::new(Filter::Null))),
                Query::Filter(Filter::Field("piyo".to_string(), Box::new(Filter::Null)))
            ]))
        );
    }

    #[test]
    fn test_parse_query3() {
        assert_eq!(
            parse_query("{\"hoge\":[],\"piyo\":[]}"),
            Ok(Query::Object(vec![
                ("hoge".to_string(), Query::Array(vec![])),
                ("piyo".to_string(), Query::Array(vec![]))
            ]))
        );
    }

    #[test]
    fn test_parse_query_spaces1() {
        assert_eq!(parse_query(" [ ] "), Ok(Query::Array(vec![])));
    }

    #[test]
    fn test_parse_query_spaces2() {
        assert_eq!(
            parse_query(" [ . hoge , . piyo ] "),
            Ok(Query::Array(vec![
                Query::Filter(Filter::Field("hoge".to_string(), Box::new(Filter::Null))),
                Query::Filter(Filter::Field("piyo".to_string(), Box::new(Filter::Null)))
            ]))
        );
    }

    #[test]
    fn test_parse_query_spaces3() {
        assert_eq!(
            parse_query(" { \"hoge\" : [ ] , \"piyo\" : [ ] } "),
            Ok(Query::Object(vec![
                ("hoge".to_string(), Query::Array(vec![])),
                ("piyo".to_string(), Query::Array(vec![]))
            ]))
        );
    }
}
