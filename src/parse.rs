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
}

#[allow(dead_code)]
pub fn json_to_sqf(str_json: &str) -> Result<String, serde_json::Error> {
    let json: Value = match serde_json::from_str(str_json) {
        Ok(j) => j,
        Err(_) => return Ok(String::from("[nil]")),
    };

    let mut sqf_vec: Vec<String> = Vec::new();

    if json.is_array() {
        json_to_sqf_parse_array(&json, &mut sqf_vec);
    }

    if json.is_boolean() {
        sqf_vec.push(json.to_string());
    }

    if json.is_number() || json.is_string() {
        sqf_vec.push(json.to_string());
    }

    if json.is_null() {
        sqf_vec.push(String::from("nil"));
    }

    if json.is_object() {
        json_to_sqf_parse_object(&json, &mut sqf_vec);
    }

    let str_sqf = make_sqf_array_string(&sqf_vec);

    Ok(str_sqf)
}

/**
 * [[["alive", true],["health", 0.5]],[["health", 0.5]],[["name", "CreepPork"]]]
 * [{"alive": true, "health": 0.5},{"health": 0.5},{"name": "CreepPork"}]
 */
#[allow(dead_code)]
fn sqf_to_json_parse_array(str_sqf: &str) -> String {
    let mut str_json = String::from(str_sqf);

    // Drop both []
    str_json.remove(0);
    str_json.pop();

    // Replace all other [] with {} (except the outer ones, those we remove and add back in)
    str_json.remove(0);
    str_json.pop();

    str_json = str_json.replace("[", "{");
    str_json = str_json.replace("]", "}");

    str_json.insert_str(0, "[");
    str_json.push_str("]");

    // Inside the arrays transform the , to :
    str_json = str_json.replace(r#"","#, r#"":"#);

    // Replace object [] with {}
    // str_json = str_json.replace(r#"[{""#, r#"[[{""#);
    str_json = str_json.replace(r#"},{""#, r#",""#);
    str_json = str_json.replace(r#"}},{{""#, r#"},{""#);

    str_json
}

/**
 * [["alive", true],["health", 0.5],["name", "CreepPork"]]
 * {"name": "CreepPork", "alive": true, "health": 0.5}
 */
#[allow(dead_code)]
fn sqf_to_json_parse_object(str_sqf: &str) -> String {
    let str_json = String::from(str_sqf);

    // TODO
    str_json
}

#[allow(dead_code)]
pub fn sqf_to_json(str_sqf: &str) -> Result<String, serde_json::Error> {
    let mut str_json = String::from("{}");

    // If has 3 arrays then drop the first
    if str_sqf.starts_with("[[[") {
        str_json = sqf_to_json_parse_array(str_sqf);
    }

    // If has 2 arrays, drop 1
    if str_sqf.starts_with(r#"[[""#) {
        str_json = sqf_to_json_parse_object(str_sqf);
    }
    Ok(str_json)
}

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

    #[test]
    fn it_will_parse_any_number_to_sqf() {
        assert_eq!(json_to_sqf("1").unwrap(), "[1]");
        assert_eq!(json_to_sqf("0.23").unwrap(), "[0.23]");
    }

    #[test]
    fn it_will_parse_undefined_and_null_to_sqf() {
        assert_eq!(json_to_sqf("undefined").unwrap(), "[nil]");
        assert_eq!(json_to_sqf("null").unwrap(), "[nil]");
    }

    #[test]
    fn it_will_parse_sqf_array_to_json_array() {
        let sqf =
            r#"[[["alive", true],["health", 0.5]],[["health", 0.5]],[["name", "CreepPork"]]]"#;
        let json = r#"[{"alive": true,"health": 0.5},{"health": 0.5},{"name": "CreepPork"}]"#;

        assert_eq!(sqf_to_json(sqf).unwrap(), json);
    }
}
