use json_schema::deserializer::{deserialize_array, impl_from_str::DeserializeToolCallParam};

#[test]
fn test_string_is_unescaped() {
    let result =
        <String as DeserializeToolCallParam>::from_str(r#""line1\nline2 \"quoted\" \u00e9""#).unwrap();

    assert_eq!(result, "line1\nline2 \"quoted\" é");
}

#[test]
fn test_single_value_as_array() {
    let result: Vec<String> = deserialize_array(r#""MyItem""#).unwrap();

    assert_eq!(result, vec!["MyItem".to_string()]);
}

#[test]
fn test_broken_array_is_error() {
    let result = deserialize_array::<String>(r#"["a", "b" "c"]"#);

    assert!(result.is_err());
}
