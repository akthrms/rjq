use crate::parser::{Filter, Query};
use serde_json::{Map, Value};

fn apply_filter(filter: Filter, value: Value) -> Result<Value, String> {
    match (filter, value) {
        (Filter::Field(field, filter), Value::Object(object)) => object
            .get(field.as_str())
            .map_or(Err(format!("field name not found: {}", field)), |value| {
                apply_filter(*filter, value.clone())
            }),

        (Filter::Index(index, filter), Value::Array(array)) => array
            .get(index)
            .map_or(Err(format!("out of range: {}", index)), |value| {
                apply_filter(*filter, value.clone())
            }),

        (Filter::Null, value) => Ok(value),

        (filter, value) => Err(format!("invalid pattern: {:?}: {:?}", filter, value)),
    }
}

pub fn execute_query(query: Query, value: Value) -> Result<Value, String> {
    match (query, value) {
        (Query::Object(object), value) => {
            let mut values = Map::new();
            for (field, query) in object {
                values.insert(field, execute_query(query, value.clone())?);
            }
            Ok(Value::Object(values))
        }

        (Query::Array(array), value) => {
            let mut values = vec![];
            for query in array {
                values.push(execute_query(query, value.clone())?);
            }
            Ok(Value::Array(values))
        }

        (Query::Filter(filter), value) => apply_filter(filter, value),
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{parse_filter, parse_query, Filter, Query};
    use crate::query::{apply_filter, execute_query};
    use serde_json::{json, Value};

    fn test_json() -> Value {
        json!({
            "string-field": "string value",
            "nested-field": {
                "inner-string": "inner value",
                "inner-number": 100
            },
            "array-field": [
                "first field",
                "next field",
                {
                    "object-in-array": "string value in object-in-array"
                }
            ]
        })
    }

    fn unsafe_parse_filter(input: &str) -> Filter {
        parse_filter(input).unwrap()
    }

    fn unsafe_parse_query(input: &str) -> Query {
        parse_query(input).unwrap()
    }

    #[test]
    fn apply_filter_test1() {
        assert_eq!(
            apply_filter(unsafe_parse_filter("."), test_json()).unwrap(),
            test_json()
        )
    }

    #[test]
    fn apply_filter_test2() {
        assert_eq!(
            apply_filter(unsafe_parse_filter(".string-field"), test_json()).unwrap(),
            test_json()["string-field"]
        )
    }

    #[test]
    fn apply_filter_test3() {
        assert_eq!(
            apply_filter(
                unsafe_parse_filter(".nested-field.inner-string"),
                test_json()
            )
            .unwrap(),
            test_json()["nested-field"]["inner-string"]
        )
    }

    #[test]
    fn apply_filter_test4() {
        assert_eq!(
            apply_filter(
                unsafe_parse_filter(".nested-field.inner-number"),
                test_json()
            )
            .unwrap(),
            test_json()["nested-field"]["inner-number"]
        )
    }

    #[test]
    fn apply_filter_test5() {
        assert_eq!(
            apply_filter(unsafe_parse_filter(".array-field[0]"), test_json()).unwrap(),
            test_json()["array-field"][0]
        )
    }

    #[test]
    fn apply_filter_test6() {
        assert_eq!(
            apply_filter(unsafe_parse_filter(".array-field[1]"), test_json()).unwrap(),
            test_json()["array-field"][1]
        )
    }

    #[test]
    fn apply_filter_test7() {
        assert_eq!(
            apply_filter(
                unsafe_parse_filter(".array-field[2].object-in-array"),
                test_json()
            )
            .unwrap(),
            test_json()["array-field"][2]["object-in-array"]
        )
    }

    #[test]
    fn execute_query_test1() {
        assert_eq!(
            execute_query(unsafe_parse_query("{}"), test_json()).unwrap(),
            json!({})
        )
    }

    #[test]
    fn execute_query_test2() {
        assert_eq!(
            execute_query(
                unsafe_parse_query("{\"field1\":.,\"field2\":.string-field}"),
                test_json()
            )
            .unwrap(),
            json!({
                "field1": test_json(),
                "field2": test_json()["string-field"]
            })
        )
    }

    #[test]
    fn execute_query_test3() {
        assert_eq!(
            execute_query(
                unsafe_parse_query("[.string-field,.nested-field.inner-string]"),
                test_json()
            )
            .unwrap(),
            json!([
                test_json()["string-field"],
                test_json()["nested-field"]["inner-string"]
            ])
        )
    }
}
