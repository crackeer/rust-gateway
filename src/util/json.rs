use serde_json::{json, Value};

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

pub fn get_json_value(src_input: &Value, path : &str) -> Option<Value> {
    if let Some(data) = src_input.pointer(path) {
        return Some(data.clone());
    }
    None
}

pub fn extract_value(src_input: &Value, dest_value: &Value) -> Option<Value> {
    if dest_value.is_string() {
        let path = value_to_string(&dest_value);
        if path.starts_with("#") {
            let pos = path.find("#").unwrap();
            let (_, real_path) = path.split_at(pos + 1);
            return get_json_value(&src_input, real_path);
        }
    }

    if !dest_value.is_object() {
        return Some(dest_value.clone());
    }

    let mut ret: Value = json!({});

    if let Some(obj) = dest_value.as_object() {
        for (key, val) in obj.into_iter() {
            if let Some(tmp) = extract_value(&src_input.clone(), &val.clone()) {
                ret.as_object_mut()
                    .unwrap()
                    .insert(String::from(key), tmp.clone());
            }
        }
    }

    Some(ret)
}
