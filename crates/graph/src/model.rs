use std::collections::BTreeMap;
use std::convert::TryFrom;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;
use uuid::Uuid;

use crate::property_value::PropertyValue;

pub type Properties = BTreeMap<String, PropertyValue>;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EdgeType {
    Parent = 0,
}

impl EdgeType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Parent => "parent",
        }
    }
}

impl TryFrom<u8> for EdgeType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Parent),
            _ => Err(()),
        }
    }
}

impl TryFrom<&str> for EdgeType {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "parent" => Ok(Self::Parent),
            _ => Err(()),
        }
    }
}

impl Serialize for EdgeType {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u8(*self as u8)
    }
}

impl<'de> Deserialize<'de> for EdgeType {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let byte = u8::deserialize(deserializer)?;
        Self::try_from(byte).map_err(|_| serde::de::Error::custom("invalid edge type"))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Block {
    pub id: Uuid,
    pub properties: Properties,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Edge {
    pub source: Uuid,
    pub target: Uuid,
    #[serde(rename = "type")]
    pub edge_type: EdgeType,
    pub properties: Properties,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum GraphError {
    #[error("block not found: {0}")]
    BlockNotFound(Uuid),
    #[error("invalid knowledge base: {0}")]
    InvalidGraph(String),
}
