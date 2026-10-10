use json_schema::JsonTypeDescription;

#[tokio::test]
async fn test_number() {
    let result = u32::get_description(false, None, false).await.build();
    assert_eq!(result, r#"{"type":"integer"}"#);
}

#[tokio::test]
async fn test_float() {
    let result = f64::get_description(false, None, false).await.build();
    assert_eq!(result, r#"{"type":"number"}"#);
}

#[tokio::test]
async fn test_string_with_enum() {
    let result = String::get_description(false, Some(vec!["a".into(), "b".into()]), false)
        .await
        .build();
    assert_eq!(result, r#"{"type":"string","enum":["a","b"]}"#);
}

#[tokio::test]
async fn test_number_with_enum() {
    let result = u8::get_description(false, Some(vec!["1".into(), "2".into()]), false)
        .await
        .build();
    assert_eq!(result, r#"{"type":"integer","enum":[1,2]}"#);
}

#[tokio::test]
async fn test_enum_value_not_of_the_type_stays_string() {
    let result = f64::get_description(false, Some(vec!["1.5".into(), "NaN".into()]), false)
        .await
        .build();
    assert_eq!(result, r#"{"type":"number","enum":[1.5,"NaN"]}"#);
}

#[tokio::test]
async fn test_option_input() {
    let result = Option::<bool>::get_description(false, None, false)
        .await
        .build();
    assert_eq!(
        result,
        r#"{"type":"boolean","nullable":true,"default":null}"#
    );
}

#[tokio::test]
async fn test_option_input_with_default() {
    let result = Option::<bool>::get_description(true, None, false)
        .await
        .build();
    assert_eq!(result, r#"{"type":"boolean","nullable":true}"#);
}

#[tokio::test]
async fn test_option_output() {
    let result = Option::<bool>::get_description(false, None, true)
        .await
        .build();
    assert_eq!(
        result,
        r#"{"anyOf":[{"type":"null"},{"type":"boolean"}],"default":null}"#
    );
}

#[tokio::test]
async fn test_vec() {
    let result = Vec::<i64>::get_description(false, None, false)
        .await
        .build();
    assert_eq!(result, r#"{"type":"array","items":{"type":"integer"}}"#);
}

#[tokio::test]
async fn test_vec_of_options() {
    let result = Vec::<Option<i32>>::get_description(false, None, false)
        .await
        .build();
    assert_eq!(
        result,
        r#"{"type":"array","items":{"type":"integer","nullable":true}}"#
    );
}

#[tokio::test]
async fn test_str_ref() {
    let result = <&str>::get_description(false, None, false).await.build();
    assert_eq!(result, r#"{"type":"string"}"#);
}
