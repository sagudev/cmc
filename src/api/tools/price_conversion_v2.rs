use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::ops::Index;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PCv2Symbol {
    pub status: Status,
    pub data: SomeConversionResult,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SomeConversionResult {
    Map(HashMap<String, ConversionResult>),
    Array(Vec<ConversionResult>),
}

impl Index<usize> for SomeConversionResult {
    type Output = ConversionResult;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            SomeConversionResult::Map(hash_map) => hash_map.iter().nth(index).unwrap().1,
            SomeConversionResult::Array(vec) => vec.get(index).unwrap(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PCv2Id {
    pub status: Status,
    pub data: ConversionResult,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Status {
    pub timestamp: String,
    pub error_code: i64,
    pub error_message: Value,
    pub elapsed: i64,
    pub credit_count: i64,
    pub notice: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConversionResult {
    pub id: Value,
    pub symbol: String,
    pub name: String,
    pub amount: f64,
    pub quote: HashMap<String, Price>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Price {
    pub price: Option<f64>,
}
