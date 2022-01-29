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
mod tests {}
