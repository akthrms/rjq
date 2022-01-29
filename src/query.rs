use crate::parser::Filter;
use serde_json::Value;

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

        (filter, value) => Err(format!("unexpected pattern: {:?}: {:?}", filter, value)),
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{parse_filter, Filter};
    use crate::query::apply_filter;
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
        match parse_filter(input) {
            Ok((_, filter)) => filter,
            _ => panic!(),
        }
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
}
