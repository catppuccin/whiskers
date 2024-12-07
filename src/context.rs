/// Recursively merge two tera values into one.
#[must_use]
pub fn merge_values(a: &tera::Value, b: &tera::Value) -> tera::Value {
    match (a, b) {
        // if both are objects, merge them
        (tera::Value::Object(a), tera::Value::Object(b)) => {
            let mut result = a.clone();
            for (k, v) in b {
                result.insert(
                    k.clone(),
                    merge_values(a.get(k).unwrap_or(&tera::Value::Null), v),
                );
            }
            tera::Value::Object(result)
        }
        // otherwise, use the second value
        (_, b) => b.clone(),
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_merge_values() {
        let a = tera::to_value(json!({
            "a": 1,
            "b": {
                "c": 2,
                "d": 3,
            },
        }))
        .expect("test value is always valid");
        let b = tera::to_value(json!({
            "b": {
                "c": 4,
                "e": 5,
            },
            "f": 6,
        }))
        .expect("test value is always valid");
        let result = merge_values(&a, &b);
        assert_eq!(
            result,
            tera::to_value(json!({
                "a": 1,
                "b": {
                    "c": 4,
                    "d": 3,
                    "e": 5,
                },
                "f": 6,
            }))
            .expect("test value is always valid")
        );
    }
}
