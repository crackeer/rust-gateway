use serde_json::{Value};

pub fn value_to_string(value : &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Number(n) => format!("{}", n),
        Value::Bool(b) => {
            if *b {
                "true".to_string()
            } else {
                "false".to_string()
            }
        }
        Value::Null => "".to_string(),
        _ => value.to_string(),
    }
}

pub fn extract_json_value<'a>(input: Option<&'a Value>, parts: Vec<&str>, index: usize) -> Option<&'a Value> {
    if !input.is_none() || parts.is_empty() {
        return None;
    }
    if !input.unwrap().is_object() {
        return None
    }

    let obj = input.unwrap().as_object().unwrap();
    if parts.len() -1 ==  index {
        return obj.get(parts[index]);
    }

    return extract_json_value(obj.get(parts[0]), parts, index + 1)

}