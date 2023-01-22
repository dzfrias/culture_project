use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Dataset {
    pub label: String,
    pub data: Vec<f32>,
    pub bg_color: String,
    pub border_color: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Chart {
    pub datasets: Vec<Dataset>,
    pub start: i32,
    pub end: i32,
    pub step: usize,
    pub opts: HashMap<String, Value>,
}
