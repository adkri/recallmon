use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct VectorRecord {
    pub vector: Vec<f32>,
    pub payload: serde_json::Value,
}
