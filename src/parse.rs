use serde_json::Value;

pub fn json_to_sqf(str_json: &str) -> Result<String, serde_json::Error> {
    let json: Value = serde_json::from_str(str_json)?;

    let mut sqf_vec: Vec<String> = Vec::new();

    for (key, value) in json.as_object().unwrap() {
        sqf_vec.push(format!(r#"["{}", {}]"#, &key, &value.to_string()));
    }

    let mut sqf_string = String::from("[");
    let sqf_vec_last_item_index = sqf_vec.len() - 1;

    for (i, s) in sqf_vec.iter().enumerate() {
        sqf_string.push_str(s);

        // Add , if not the last item in the array
        if sqf_vec_last_item_index != i {
            sqf_string.push_str(",");
        }
    }
    sqf_string.push_str("]");

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
}
