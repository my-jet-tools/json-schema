pub fn deserialize_array<
    TResult: crate::deserializer::impl_from_str::DeserializeToolCallParam,
>(
    src: &str,
) -> Result<Vec<TResult>, String> {
    // A single value where an array is expected is taken as an array of one item.
    // `JsonArrayIterator::new` does not check for `[`, so it can not tell this case from a broken array.
    if !src.trim_start().starts_with('[') {
        let item = TResult::from_str(src)?;
        return Ok(vec![item]);
    }

    let array_iterator = my_json::json_reader::JsonArrayIterator::new(src.as_bytes())
        .map_err(|err| format!("Can not deserialize array {src}: {:?}", err))?;

    let mut result = vec![];

    while let Some(item) = array_iterator.get_next() {
        let item = item.map_err(|err| format!("Can not deserialize array {src}: {:?}", err))?;

        if let Some(raw_str) = item.as_raw_str() {
            result.push(TResult::from_str(raw_str)?);
        }
    }

    Ok(result)
}
