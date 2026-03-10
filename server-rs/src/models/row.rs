// server-rs\src\models\row.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DemoRow {
    pub name: String,
    pub client: String,
    #[serde(rename = "lastUpdate")]
    pub last_update: i64,
    pub chg: f64,
    pub bid: f64,
    pub ask: f64,
    pub vol: f64,
}