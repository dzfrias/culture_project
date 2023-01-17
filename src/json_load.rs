use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Dataset {
    pub label: String,
    pub data: Vec<i32>,
    pub bg_color: String,
    pub border_color: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Chart {
    pub datasets: Vec<Dataset>,
    pub start: i32,
    pub end: i32,
    pub step: usize,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Side {
    Left,
    Right,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Alignment {
    Start,
    Center,
    End,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Quote<'a> {
    pub text: &'a str,
    pub side: Side,
    pub alignment: Alignment,
    pub top: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Quotes<'a> {
    #[serde(borrow)]
    pub quotes: Vec<Quote<'a>>,
}
