use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Count {
    pub(crate) timestamp_ms: u64,
    pub(crate) count: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ScrapCount {
    pub(crate) timestamp_ms: u64,
    pub(crate) scrap: u64,
}
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Barcode {
    pub(crate) timestamp_ms: u64,
    pub(crate) barcode: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Activity {
    pub(crate) timestamp_ms: u64,
    pub(crate) activity: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct DetectedAnomaly {
    pub(crate) timestamp_ms: u64,
    #[serde(rename = "detectedAnomaly")]
    pub(crate) detected_anomaly: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AddShift {
    pub(crate) timestamp_ms: u64,
    pub(crate) timestamp_ms_end: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AddOrder {
    pub(crate) product_id: String,
    pub(crate) order_id: String,
    pub(crate) target_units: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AddProduct {
    pub(crate) product_id: u64,
    pub(crate) time_per_unit_in_seconds: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct StartOrder {
    pub(crate) timestamp_ms: u64,
    pub(crate) order_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct EndOrder {
    pub(crate) timestamp_ms: u64,
    pub(crate) order_id: String,
}
