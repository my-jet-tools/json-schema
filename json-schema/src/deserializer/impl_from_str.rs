use std::str::FromStr;

use my_json::json_string_value::de_escape_json_string_value;
use rust_extensions::str_utils::StrUtils;

pub trait DeserializeToolCallParam {
    fn from_str(src: &str) -> Result<Self, String>
    where
        Self: Sized;
}

macro_rules! impl_for_numbers {
    ($($tp:ty),+) => {
        $(
            impl DeserializeToolCallParam for $tp {
                fn from_str(src: &str) -> Result<Self, String> {
                    number_from_str(src)
                }
            }
        )+
    };
}

impl_for_numbers!(u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64);

impl DeserializeToolCallParam for String {
    fn from_str(src: &str) -> Result<Self, String>
    where
        Self: Sized,
    {
        // A JSON string comes quoted and escaped; any other value is taken as is
        match src.strip_prefix('"').and_then(|src| src.strip_suffix('"')) {
            Some(value) => Ok(de_escape_json_string_value(value).into_string()),
            None => Ok(src.to_string()),
        }
    }
}

impl DeserializeToolCallParam for bool {
    fn from_str(src: &str) -> Result<Self, String>
    where
        Self: Sized,
    {
        if src.eq_case_insensitive("true") {
            return Ok(true);
        }

        if src.eq_case_insensitive("false") {
            return Ok(false);
        }

        return Err(format!("Can not deserialize value `{src}` to boolean type"));
    }
}

fn number_from_str<T: FromStr>(src: &str) -> Result<T, String> {
    match src.parse() {
        Ok(result) => return Ok(result),
        Err(_) => {
            return Err(format!("Can not convert to number from str {src}"));
        }
    }
}
