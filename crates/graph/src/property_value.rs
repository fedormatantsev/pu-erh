use std::fmt;

use serde::de::{self, Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeMap, Serializer};

#[derive(Debug, Clone, PartialEq)]
pub enum PropertyValue {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
}

impl PropertyValue {
    pub fn digest_bytes(&self) -> Vec<u8> {
        match self {
            Self::Null => vec![0],
            Self::Boolean(value) => vec![1, u8::from(*value)],
            Self::Number(value) => {
                let mut bytes = vec![2];
                bytes.extend_from_slice(&value.to_le_bytes());
                bytes
            }
            Self::String(value) => {
                let utf8 = value.as_bytes();
                let mut bytes = vec![3];
                bytes.extend_from_slice(&(utf8.len() as u64).to_le_bytes());
                bytes.extend_from_slice(utf8);
                bytes
            }
        }
    }

    fn append_json(&self, out: &mut String) {
        match self {
            Self::Null => out.push_str("null"),
            Self::Boolean(value) => out.push_str(if *value { "true" } else { "false" }),
            Self::Number(value) => append_json_number(out, *value),
            Self::String(value) => {
                out.push('"');
                append_json_escaped(out, value);
                out.push('"');
            }
        }
    }
}

impl Serialize for PropertyValue {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            // TOML (and some other formats) cannot represent serde unit; use a sentinel table.
            Self::Null => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("null", &true)?;
                map.end()
            }
            Self::Boolean(value) => serializer.serialize_bool(*value),
            Self::Number(value) => serializer.serialize_f64(*value),
            Self::String(value) => serializer.serialize_str(value),
        }
    }
}

impl<'de> Deserialize<'de> for PropertyValue {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(PropertyValueVisitor)
    }
}

struct PropertyValueVisitor;

impl<'de> Visitor<'de> for PropertyValueVisitor {
    type Value = PropertyValue;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a JSON string, number, boolean, or null")
    }

    fn visit_bool<E: de::Error>(self, value: bool) -> Result<Self::Value, E> {
        Ok(PropertyValue::Boolean(value))
    }

    fn visit_i64<E: de::Error>(self, value: i64) -> Result<Self::Value, E> {
        Ok(PropertyValue::Number(value as f64))
    }

    fn visit_u64<E: de::Error>(self, value: u64) -> Result<Self::Value, E> {
        Ok(PropertyValue::Number(value as f64))
    }

    fn visit_f64<E: de::Error>(self, value: f64) -> Result<Self::Value, E> {
        Ok(PropertyValue::Number(value))
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        Ok(PropertyValue::String(value.to_owned()))
    }

    fn visit_string<E: de::Error>(self, value: String) -> Result<Self::Value, E> {
        Ok(PropertyValue::String(value))
    }

    fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok(PropertyValue::Null)
    }

    fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok(PropertyValue::Null)
    }

    fn visit_some<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        Deserialize::deserialize(deserializer)
    }

    fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut null_marker = false;
        let mut other_keys = 0usize;
        while let Some(key) = map.next_key::<String>()? {
            if key == "null" {
                let value: bool = map.next_value()?;
                if value {
                    null_marker = true;
                } else {
                    return Err(de::Error::custom("null marker must be true"));
                }
            } else {
                other_keys += 1;
                let _: de::IgnoredAny = map.next_value()?;
            }
        }
        if null_marker && other_keys == 0 {
            Ok(PropertyValue::Null)
        } else {
            Err(de::Error::custom("object property values are not supported"))
        }
    }

    fn visit_seq<A: SeqAccess<'de>>(self, _seq: A) -> Result<Self::Value, A::Error> {
        Err(de::Error::custom("array property values are not supported"))
    }
}

pub fn properties_to_json_string(properties: &super::Properties) -> String {
    if properties.is_empty() {
        return "{}".to_string();
    }
    let mut out = String::from("{");
    for (index, (key, value)) in properties.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        out.push('"');
        append_json_escaped(&mut out, key);
        out.push('"');
        out.push(':');
        value.append_json(&mut out);
    }
    out.push('}');
    out
}

fn append_json_number(out: &mut String, value: f64) {
    if value.is_finite() && value.fract() == 0.0 {
        let as_i64 = value as i64;
        if (as_i64 as f64) == value {
            out.push_str(&as_i64.to_string());
            return;
        }
    }
    out.push_str(&value.to_string());
}

fn append_json_escaped(out: &mut String, value: &str) {
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c.is_control() => {
                out.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => out.push(c),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digest_bytes_null() {
        assert_eq!(PropertyValue::Null.digest_bytes(), vec![0]);
    }

    #[test]
    fn digest_bytes_boolean() {
        assert_eq!(PropertyValue::Boolean(true).digest_bytes(), vec![1, 1]);
        assert_eq!(PropertyValue::Boolean(false).digest_bytes(), vec![1, 0]);
    }

    #[test]
    fn digest_bytes_number() {
        let bytes = PropertyValue::Number(1.5).digest_bytes();
        assert_eq!(bytes[0], 2);
        assert_eq!(&bytes[1..], &1.5f64.to_le_bytes());
    }

    #[test]
    fn digest_bytes_string() {
        let bytes = PropertyValue::String("hi".into()).digest_bytes();
        assert_eq!(bytes[0], 3);
        assert_eq!(&bytes[1..9], &2u64.to_le_bytes());
        assert_eq!(&bytes[9..], b"hi");
    }
}
