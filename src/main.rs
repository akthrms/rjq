use rjq::parser::parse_query;
use rjq::query::execute_query;

fn main() {
    println!("Hello, world!");
}

fn rjq(json_string: String, query_string: String) -> Result<String, String> {
    let value = serde_json::to_value(json_string).expect("invalid json format");
    let query = parse_query(query_string.as_str())?;
    execute_query(query, value).map(|value| value.to_string())
}
