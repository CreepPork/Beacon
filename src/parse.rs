use serde_json::json;
use serde_json::Value;

pub fn json_to_sqf(str_json: &str) -> Result<String, &str> {
    // todo: panik
    let json: Vec<Value> = serde_json::from_str(str_json).unwrap();

    // for j in &json {
    //     //
    // }

    let mut sqf = String::from(str_json.trim());

    // Drop { }
    sqf.remove(0);
    sqf.pop();

    // Wrap in array
    // We get this:
    // ["name": "CreepPork", "alive": true, "health": 0.5]
    sqf.insert_str(0, "[");
    sqf.push_str("]");

    // ["<key>", <value>]
    let sqf_vec: Vec<(&str, &str)> = Vec::new();

    // let split_sqf_vec: Vec<&str> = sqf.split(",")

    // Parse <value> into it's specific data type (undefined|null, number, bool, string, object, array)

    Ok(sqf.to_string())
}

// pub fn sqf_to_json(sqf: &str) -> &str {
//     ""
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_will_parse_json_to_sqf() {
        let json = r#"{"name": "CreepPork", "alive": true, "health": 0.5}"#;
        let expected_sqf = r#"[["name", "CreepPork"], ["alive", true], ["health", 0.5]]"#;

        assert_eq!(json_to_sqf(&json).unwrap(), expected_sqf);
    }
}
