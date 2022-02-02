use rjq::parser::parse_query;
use rjq::query::execute_query;
use std::env;
use std::fs;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() == 3 {
        match rjq(
            fs::read_to_string(&args[2])
                .expect("invalid file path")
                .as_str(),
            &args[1],
        ) {
            Ok(s) => println!("{}", s),
            Err(e) => panic!("{}", e),
        }
    } else {
        panic!("invalid arguments")
    }
}

fn rjq(json_string: &str, query_string: &str) -> Result<String, String> {
    let value = serde_json::from_str(json_string).expect("invalid json format");
    let query = parse_query(query_string)?;
    execute_query(query, value).map(|value| value.to_string())
}

#[cfg(test)]
mod tests {
    use crate::rjq;
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

    #[test]
    fn rjq_test1() {
        assert_eq!(
            rjq(test_json().to_string().as_str(), "{}"),
            Ok(json!({}).to_string())
        )
    }

    #[test]
    fn rjq_test2() {
        assert_eq!(
            rjq(
                test_json().to_string().as_str(),
                "{\"field1\":.,\"field2\":.string-field}"
            ),
            Ok(json!({
                "field1": test_json(),
                "field2": test_json()["string-field"]
            })
            .to_string())
        )
    }

    #[test]
    fn rjq_test3() {
        assert_eq!(
            rjq(
                test_json().to_string().as_str(),
                "[.string-field,.nested-field.inner-string]"
            ),
            Ok(json!([
                test_json()["string-field"],
                test_json()["nested-field"]["inner-string"]
            ])
            .to_string())
        )
    }
}
