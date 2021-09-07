use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Count {
    pub(crate) timestamp_ms: i64,
    pub(crate) count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ScrapCount {
    pub(crate) timestamp_ms: i64,
    pub(crate) scrap: i32,
}
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Barcode {
    pub(crate) timestamp_ms: i64,
    pub(crate) barcode: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Activity {
    pub(crate) timestamp_ms: i64,
    pub(crate) activity: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct DetectedAnomaly {
    pub(crate) timestamp_ms: i64,
    #[serde(rename = "detectedAnomaly")]
    pub(crate) detected_anomaly: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AddShift {
    pub(crate) timestamp_ms: i64,
    pub(crate) timestamp_ms_end: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AddOrder {
    pub(crate) product_id: String,
    pub(crate) order_id: String,
    pub(crate) target_units: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AddProduct {
    pub(crate) product_id: i32,
    pub(crate) time_per_unit_in_seconds: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct StartOrder {
    pub(crate) timestamp_ms: i64,
    pub(crate) order_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct EndOrder {
    pub(crate) timestamp_ms: i64,
    pub(crate) order_id: String,
}
