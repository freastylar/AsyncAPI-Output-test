use super::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TelemetryMessage {
    pub active: bool,
    pub humidity: i32,
    #[serde(rename = "sensorId")]
    pub sensor_id: String,
    pub temperature: f32,
    pub timestamp: String,
}
