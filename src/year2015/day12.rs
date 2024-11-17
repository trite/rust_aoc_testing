use serde_json::Value;

pub fn part_1(input: &str) -> i32 {
    let json: Value = serde_json::from_str(input).unwrap();
    sum_integers(&json)
}

pub fn part_2(input: &str) -> i32 {
    let json: Value = serde_json::from_str(input).unwrap();
    sum_integers(&filter_red_objects(&json))
}

fn filter_red_objects(value: &Value) -> Value {
    match value {
        Value::Object(obj) => {
            if obj.values().any(|v| v == "red") {
                Value::Null
            } else {
                Value::Object(
                    obj.iter()
                        .map(|(k, v)| (k.clone(), filter_red_objects(v)))
                        .collect(),
                )
            }
        }
        Value::Array(arr) => {
            Value::Array(arr.iter().map(filter_red_objects).collect())
        }
        _ => value.clone(),
    }
}

fn sum_integers(value: &Value) -> i32 {
    match value {
        Value::Number(n) => n.as_i64().unwrap_or(0) as i32,
        Value::Array(arr) => arr.iter().map(sum_integers).sum(),
        Value::Object(obj) => obj.values().map(sum_integers).sum(),
        _ => 0,
    }
}

generate_tests!(
    2015,
    12,
    part_1,
    part_2,
    vec![
        ("[1,2,3]", 6),
        ("{\"a\":2,\"b\":4}", 6),
        ("[[[3]]]", 3),
        ("{\"a\":{\"b\":4},\"c\":-1}", 3),
        ("{\"a\":[-1,1]}", 0),
        ("[-1,{\"a\":1}]", 0),
        ("[]", 0),
        ("{}", 0),
    ],
    vec![
        ("[1,2,3]", 6),
        ("[1,{\"c\":\"red\",\"b\":2},3]", 4),
        ("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}", 0),
        ("[1,\"red\",5]", 6),
    ],
    Some(191164),
    Some(87842)
);
