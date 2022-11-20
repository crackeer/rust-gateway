use serde_json::{json, Value};
use std::collections::HashMap;

pub fn value_to_string(value: &Value) -> String {
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

/* 
pub fn extract_json_value<'a>(
    input: Option<&'a Value>,
    parts: Vec<&str>,
    index: usize,
) -> Option<&'a Value> {
    if !input.is_none() || parts.is_empty() {
        return None;
    }
    if !input.unwrap().is_object() {
        return None;
    }

    let obj = input.unwrap().as_object().unwrap();
    if parts.len() - 1 == index {
        return obj.get(parts[index]).to_owned();
    }

    return extract_json_value(obj.get(parts[0]), parts, index + 1);
}*/





pub fn extract_value(src_input: Value, dest_value: Value) -> Option<Value> {
    if dest_value.is_string() {
        let data = src_input
            .pointer(dest_value.as_str().unwrap())
            .unwrap()
            .clone();
        return Some(data);
    }

    if !dest_value.is_object() {
        return Some(dest_value.clone());
    }

    let mut ret: Value = json!({});

    if let Some(obj) = dest_value.as_object() {
        for (key, val) in obj.into_iter() {
            if let Some(tmp) = extract_value(src_input.clone(), val.clone()) {
                ret.as_object_mut()
                    .unwrap()
                    .insert(String::from(key), tmp.clone());
            }
        }
    }

    Some(ret)
}
