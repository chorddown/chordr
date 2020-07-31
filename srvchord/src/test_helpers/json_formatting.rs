use std::convert::From;
use std::fmt;

pub enum JsonTemplateValue {
    Text(String),
    PosInt(u64),
    NegInt(i64),
    Float(f64),
}

impl JsonTemplateValue {}

impl fmt::Display for JsonTemplateValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                JsonTemplateValue::Text(s) => s.clone(),
                JsonTemplateValue::PosInt(i) => i.to_string(),
                JsonTemplateValue::NegInt(i) => i.to_string(),
                JsonTemplateValue::Float(i) => i.to_string(),
            }
        )
    }
}

impl From<u32> for JsonTemplateValue {
    fn from(v: u32) -> Self {
        JsonTemplateValue::PosInt(v as u64)
    }
}

impl From<i32> for JsonTemplateValue {
    fn from(v: i32) -> Self {
        JsonTemplateValue::NegInt(v as i64)
    }
}

impl From<&str> for JsonTemplateValue {
    fn from(v: &str) -> Self {
        JsonTemplateValue::Text(v.to_owned())
    }
}

impl From<String> for JsonTemplateValue {
    fn from(v: String) -> Self {
        JsonTemplateValue::Text(v)
    }
}

pub fn json_format<T: Into<JsonTemplateValue>>(format: &str, values: Vec<T>) -> String {
    let mut output = format.to_owned();
    for value in values {
        output = output.replacen("$", &value.into().to_string(), 1);
    }

    output
}
