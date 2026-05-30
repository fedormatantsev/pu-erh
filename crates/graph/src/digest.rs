use blake3::Hasher;
use serde::{Deserialize, Deserializer, Serializer};
use thiserror::Error;
use uuid::Uuid;

use crate::model::EdgeType;
use crate::Properties;

pub type Digest = [u8; 32];

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DigestError {
    #[error("property value for key {0} is not JSON-serializable")]
    PropertyNotSerializable(String),
    #[error("digest mismatch")]
    Mismatch,
}

pub fn hash_block_content(
    id: Uuid,
    version: u64,
    tombstoned: bool,
    properties: &Properties,
) -> Result<Digest, DigestError> {
    let mut hasher = Hasher::new();
    hasher.update(id.as_bytes());
    hasher.update(&version.to_le_bytes());
    hasher.update(&[u8::from(tombstoned)]);
    hash_properties(&mut hasher, properties)?;
    Ok(*hasher.finalize().as_bytes())
}

pub fn hash_edge_content(
    source: Uuid,
    target: Uuid,
    edge_type: EdgeType,
    version: u64,
    tombstoned: bool,
    properties: &Properties,
) -> Result<Digest, DigestError> {
    let mut hasher = Hasher::new();
    hasher.update(source.as_bytes());
    hasher.update(target.as_bytes());
    hasher.update(&[edge_type as u8]);
    hasher.update(&version.to_le_bytes());
    hasher.update(&[u8::from(tombstoned)]);
    hash_properties(&mut hasher, properties)?;
    Ok(*hasher.finalize().as_bytes())
}

fn hash_properties(hasher: &mut Hasher, properties: &Properties) -> Result<(), DigestError> {
    for (key, value) in properties {
        hasher.update(key.as_bytes());
        let bytes = serde_json::to_vec(value).map_err(|_| {
            DigestError::PropertyNotSerializable(key.clone())
        })?;
        hasher.update(&bytes);
    }
    Ok(())
}

pub mod serde_hex {
    use super::*;

    pub fn serialize<S: Serializer>(digest: &Digest, serializer: S) -> Result<S::Ok, S::Error> {
        let hex = digest.iter().map(|b| format!("{b:02x}")).collect::<String>();
        serializer.serialize_str(&hex)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Digest, D::Error> {
        let hex = String::deserialize(deserializer)?;
        decode_hex(&hex).map_err(serde::de::Error::custom)
    }

    pub mod option {
        use super::*;

        pub fn serialize<S: Serializer>(
            digest: &Option<Digest>,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            match digest {
                Some(value) => serializer.serialize_some(&hex_string(value)),
                None => serializer.serialize_none(),
            }
        }

        pub fn deserialize<'de, D: Deserializer<'de>>(
            deserializer: D,
        ) -> Result<Option<Digest>, D::Error> {
            let value: Option<String> = Option::deserialize(deserializer)?;
            value
                .map(|hex| decode_hex(&hex).map_err(serde::de::Error::custom))
                .transpose()
        }
    }

    fn hex_string(digest: &Digest) -> String {
        digest.iter().map(|b| format!("{b:02x}")).collect()
    }

    fn decode_hex(hex: &str) -> Result<Digest, String> {
        if hex.len() != 64 {
            return Err(format!("expected 64 hex chars, got {}", hex.len()));
        }
        let mut out = [0u8; 32];
        for (idx, chunk) in hex.as_bytes().chunks(2).enumerate() {
            let pair = std::str::from_utf8(chunk).map_err(|err| err.to_string())?;
            out[idx] = u8::from_str_radix(pair, 16).map_err(|err| err.to_string())?;
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digest_is_stable() {
        let id = Uuid::new_v4();
        let props = Properties::new();
        let first = hash_block_content(id, 1, false, &props).unwrap();
        let second = hash_block_content(id, 1, false, &props).unwrap();
        assert_eq!(first, second);
    }

    #[test]
    fn digest_is_independent_of_property_insertion_order() {
        let id = Uuid::new_v4();
        let mut first = Properties::new();
        first.insert("b".into(), serde_json::json!(2));
        first.insert("a".into(), serde_json::json!(1));
        let mut second = Properties::new();
        second.insert("a".into(), serde_json::json!(1));
        second.insert("b".into(), serde_json::json!(2));
        let digest_first = hash_block_content(id, 1, false, &first).unwrap();
        let digest_second = hash_block_content(id, 1, false, &second).unwrap();
        assert_eq!(digest_first, digest_second);
    }
}
