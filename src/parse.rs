use serde_json::Value;

fn make_sqf_array_string(vec: &Vec<String>) -> String {
    let mut sqf_string = String::from("[");
    let vec_last_item_index = vec.len() - 1;

    for (i, s) in vec.iter().enumerate() {
        sqf_string.push_str(s);

        // Add , if not the last item in the array
        if vec_last_item_index != i {
            sqf_string.push_str(",");
        }
    }
    sqf_string.push_str("]");

    sqf_string
}

fn json_to_sqf_parse_object(json: &Value, sqf_vec: &mut Vec<String>) {
    for (key, value) in json.as_object().unwrap() {
        sqf_vec.push(format!(r#"["{}", {}]"#, &key, &value.to_string()));
    }
}

fn json_to_sqf_parse_array(json: &Value, sqf_vec: &mut Vec<String>) {
    let mut combined_vector: Vec<Vec<String>> = Vec::new();

    for item in json.as_array().unwrap() {
        let mut new_vec: Vec<String> = Vec::new();

        json_to_sqf_parse_object(item, &mut new_vec);
        combined_vector.push(new_vec);
    }

    for item in combined_vector.iter() {
        sqf_vec.push(make_sqf_array_string(item));
    }
    println!("{:?}", sqf_vec);
}

pub fn json_to_sqf(str_json: &str) -> Result<String, serde_json::Error> {
    let json: Value = serde_json::from_str(str_json)?;

    let mut sqf_vec: Vec<String> = Vec::new();

    if json.is_boolean() {
        sqf_vec.push(json.to_string());
    }

    if json.is_array() {
        json_to_sqf_parse_array(&json, &mut sqf_vec);
    }

    if json.is_object() {
        json_to_sqf_parse_object(&json, &mut sqf_vec);
    }

    let sqf_string = make_sqf_array_string(&sqf_vec);

    Ok(sqf_string)
}

// pub fn sqf_to_json(sqf: &str) -> &str {
//     ""
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_will_parse_object_json_to_sqf() {
        let json = r#"{"name": "CreepPork", "alive": true, "health": 0.5}"#;
        let expected_sqf = r#"[["alive", true],["health", 0.5],["name", "CreepPork"]]"#;

        assert_eq!(json_to_sqf(&json).unwrap(), expected_sqf);
    }

    #[test]
    fn it_will_parse_bool_to_sqf() {
        assert_eq!(json_to_sqf("true").unwrap(), r#"[true]"#);
    }

    #[test]
    fn it_will_parse_array_to_sqf() {
        let json = r#"[{"alive": true, "health": 0.5},{"health": 0.5},{"name": "CreepPork"}]"#;
        let expected_sqf =
            r#"[[["alive", true],["health", 0.5]],[["health", 0.5]],[["name", "CreepPork"]]]"#;

        assert_eq!(json_to_sqf(&json).unwrap(), expected_sqf);
    }
}
