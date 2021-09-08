use serde::Deserialize;
use serde::Serialize;

///ia/factoryinsight/testLocation/testMachine/count
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Count {
    pub(crate) timestamp_ms: u32,
    pub(crate) count: u32,
}

///ia/factoryinsight/testLocation/testMachine/scrapCount
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ScrapCount {
    pub(crate) timestamp_ms: u32,
    pub(crate) scrap: u32,
}

///ia/factoryinsight/testLocation/testMachine/barcode
/// Not yet in mqtt-to-postgres
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Barcode {
    pub(crate) timestamp_ms: u32,
    pub(crate) barcode: u32,
}

///ia/factoryinsight/testLocation/testMachine/activity
/// Not yet in mqtt-to-postgres
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Activity {
    pub(crate) timestamp_ms: u32,
    pub(crate) activity: bool,
}

///ia/factoryinsight/testLocation/testMachine/detectedAnomaly
/// Not yet in mqtt-to-postgres
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct DetectedAnomaly {
    pub(crate) timestamp_ms: u32,
    #[serde(rename = "detectedAnomaly")]
    pub(crate) detected_anomaly: String,
}

///ia/factoryinsight/testLocation/testMachine/addShift
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AddShift {
    pub(crate) timestamp_ms: u32,
    pub(crate) timestamp_ms_end: u32,
}

///ia/factoryinsight/testLocation/testMachine/addOrder
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AddOrder {
    pub(crate) product_id: String,
    pub(crate) order_id: String,
    pub(crate) target_units: u32,
}

///ia/factoryinsight/testLocation/testMachine/addProduct
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AddProduct {
    pub(crate) product_id: u32,
    pub(crate) time_per_unit_in_seconds: f64,
}

///ia/factoryinsight/testLocation/testMachine/startOrder
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct StartOrder {
    pub(crate) timestamp_ms: u32,
    pub(crate) order_id: String,
}

///ia/factoryinsight/testLocation/testMachine/endOrder
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct EndOrder {
    pub(crate) timestamp_ms: u32,
    pub(crate) order_id: String,
}

///ia/factoryinsight/testLocation/testMachine/processValueFloat64
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ProcessValueFloat64 {
    pub(crate) timestamp_ms: u32,
    pub(crate) name: String,
    pub(crate) value: f64,
}

///ia/factoryinsight/testLocation/testMachine/processValue
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ProcessValue {
    pub(crate) timestamp_ms: u32,
    pub(crate) name: String,
    pub(crate) value: u32,
}

///ia/factoryinsight/testLocation/testMachine/processValueString
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ProcessValueString {
    pub(crate) timestamp_ms: u32,
    pub(crate) name: String,
    pub(crate) value: String,
}

///ia/factoryinsight/testLocation/testMachine/recommendation
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Recommendation {
    pub(crate) timestamp_ms: u32,
    pub(crate) uid: String,
    pub(crate) customer: String,
    pub(crate) location: String,
    pub(crate) asset: String,
    #[serde(rename = "RecommendationType")]
    pub(crate) recommendation_type: i32,
    pub(crate) enabled: bool,
    #[serde(rename = "RecommendationValues")]
    pub(crate) recommendation_values: String,
    #[serde(rename = "DiagnoseTextDE")]
    pub(crate) diagnose_text_de: String,
    #[serde(rename = "DiagnoseTextEN")]
    pub(crate) diagnose_text_en: String,
    #[serde(rename = "RecommendationTextDE")]
    pub(crate) recommendation_text_de: String,
    #[serde(rename = "RecommendationTextEN")]
    pub(crate) recommendation_text_en: String,
}

///ia/factoryinsight/testLocation/testMachine/state
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct State {
    pub(crate) timestamp_ms: u32,
    pub(crate) state: u32,
}

///ia/factoryinsight/testLocation/testMachine/uniqueProduct
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct UniqueProduct {
    pub(crate) begin_timestamp_ms: u32,
    pub(crate) end_timestamp_ms: u32,
    #[serde(rename = "productID")]
    pub(crate) product_name: String,
    #[serde(rename = "isScrap")]
    pub(crate) is_scrap: bool,
    #[serde(rename = "uniqueProductAlternativeID")]
    pub(crate) unique_product_alternative_id: String,
}

///ia/factoryinsight/testLocation/testMachine/scrapUniqueProduct
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ScrapUniqueProduct {
    pub(crate) uid: String,
}

///ia/factoryinsight/testLocation/testMachine/addMaintenanceActivity
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AddMaintenanceActivity {
    pub(crate) timestamp_ms: u32,
    #[serde(rename = "component")]
    pub(crate) component_name: String,
    pub(crate) activity: i32,
}

///ia/factoryinsight/testLocation/testMachine/productTag
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ProductTag {
    pub(crate) timestamp_ms: u32,
    #[serde(rename = "AID")]
    pub(crate) aid: String,
    pub(crate) name: String,
    pub(crate) value: f64,
}

///ia/factoryinsight/testLocation/testMachine/productTagString
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ProductTagString {
    pub(crate) timestamp_ms: u32,
    #[serde(rename = "AID")]
    pub(crate) aid: String,
    pub(crate) name: String,
    pub(crate) value: String,
}

///ia/factoryinsight/testLocation/testMachine/addParentToChild
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AddParentToChild {
    pub(crate) timestamp_ms: u32,
    #[serde(rename = "childAID")]
    pub(crate) child_aid: String,
    #[serde(rename = "parentAID")]
    pub(crate) parent_aid: String,
}

///ia/factoryinsight/testLocation/testMachine/modifyState
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ModifyState {
    pub(crate) start_time_stamp: u32,
    pub(crate) end_time_stamp: u32,
    pub(crate) new_state: u32,
}

///ia/factoryinsight/testLocation/testMachine/modifyProducedPieces
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ModifyProducedPieces {
    pub(crate) count: i32,
    pub(crate) scrap: i32,
}

///ia/factoryinsight/testLocation/testMachine/deleteShiftById
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct DeleteShiftById {
    pub(crate) shift_id: u32,
}

///ia/factoryinsight/testLocation/testMachine/deleteShiftByAssetIdAndBeginTimestamp
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct DeleteShiftByAssetIdAndBeginTimestamp {
    pub(crate) begin_time_stamp: u32,
}
