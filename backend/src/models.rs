// backend/src/models.rs
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use bigdecimal::BigDecimal as JsonBigDecimal;
use sqlx::types::BigDecimal as SqlxBigDecimal;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: i32,
    #[serde(with = "bigdecimal_serde")]
    pub amount: JsonBigDecimal,
    pub category: String,
    pub description: Option<String>,
    pub date: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct NewTransaction {
    #[serde(with = "bigdecimal_serde")]
    pub amount: JsonBigDecimal,
    pub category: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionCreate {
    pub amount: f64,
    pub category: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionUpdate {
    pub amount: Option<f64>,
    pub category: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

// Helper module for serializing/deserializing BigDecimal
mod bigdecimal_serde {
    use super::*;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &JsonBigDecimal, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        value.to_string().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<JsonBigDecimal, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        JsonBigDecimal::parse_bytes(s.as_bytes(), 10)
            .ok_or_else(|| serde::de::Error::custom("invalid decimal"))
    }
}

// Helper functions for converting between BigDecimal types
pub fn json_to_sqlx_bigdecimal(value: &JsonBigDecimal) -> SqlxBigDecimal {
    SqlxBigDecimal::from_str(&value.to_string())
        .expect("Failed to convert JsonBigDecimal to SqlxBigDecimal")
}

pub fn sqlx_to_json_bigdecimal(value: &SqlxBigDecimal) -> JsonBigDecimal {
    JsonBigDecimal::from_str(&value.to_string())
        .expect("Failed to convert SqlxBigDecimal to JsonBigDecimal")
}