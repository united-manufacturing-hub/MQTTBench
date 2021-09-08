use crate::generator::models::{
    Activity, AddMaintenanceActivity, AddOrder, AddParentToChild, AddProduct, AddShift, Barcode,
    Count, DeleteShiftByAssetIdAndBeginTimestamp, DeleteShiftById, DetectedAnomaly, EndOrder,
    ModifyProducedPieces, ModifyState, ProcessValue, ProcessValueFloat64, ProcessValueString,
    ProductTag, ProductTagString, Recommendation, ScrapCount, ScrapUniqueProduct, StartOrder,
    State, UniqueProduct,
};

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

/// Generates random strings
pub(crate) fn generate_random_string(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

/// Generates random payloads
pub(crate) fn generate_random(amount: u64) -> Vec<(String, String)> {
    let mut rng = rand::thread_rng();
    let mut payloads = vec![];

    for _ in rng.gen()..amount {
        let rand_val = rng.gen_range(0..=24);

        let payload = match rand_val {
            0 => {
                let c = Count::generate_random();
                (c.0, serde_json::to_string(&c.1).unwrap())
            }
            1 => {
                let c = ScrapCount::generate_random();
                (c.0, serde_json::to_string(&c.1).unwrap())
            }
            2 => {
                let c = Barcode::generate_random();
                (c.0, serde_json::to_string(&c.1).unwrap())
            }
            3 => {
                let c = Activity::generate_random();
                (c.0, serde_json::to_string(&c.1).unwrap())
            }
            4 => {
                let c = DetectedAnomaly::generate_random();
                (c.0, serde_json::to_string(&c.1).unwrap())
            }
            5 => {
                let c = AddShift::generate_random();
                (c.0, serde_json::to_string(&c.1).unwrap())
            }
            6 => {
                let c = AddOrder::generate_random();
                (c.0, serde_json::to_string(&c.1).unwrap())
            }
            7 => {
                let c = AddProduct::generate_random();
                (c.0, serde_json::to_string(&c.1).unwrap())
            }
            8 => {
                let c = StartOrder::generate_random();
                (c.0, serde_json::to_string(&c.1).unwrap())
            }
            9 => {
                let c = EndOrder::generate_random();
                (c.0, serde_json::to_string(&c.1).unwrap())
            }
            10 => {
                let c = ProcessValueFloat64::generate_random();
                (c.0, serde_json::to_string(&c.1).unwrap())
            }
            11 => {
                let c = ProcessValue::generate_random();
                (c.0, serde_json::to_string(&c.1).unwrap())
            }
            12 => {
                let c = ProcessValueString::generate_random();
                (c.0, serde_json::to_string(&c.1).unwrap())
            }
            13 => {
                let c = Recommendation::generate_random();
                (c.0, serde_json::to_string(&c.1).unwrap())
            }
            14 => {
                let c = State::generate_random();
                (c.0, serde_json::to_string(&c.1).unwrap())
            }
            15 => {
                let c = UniqueProduct::generate_random();
                (c.0, serde_json::to_string(&c.1).unwrap())
            }
            16 => {
                let c = ScrapUniqueProduct::generate_random();
                (c.0, serde_json::to_string(&c.1).unwrap())
            }
            17 => {
                let c = AddMaintenanceActivity::generate_random();
                (c.0, serde_json::to_string(&c.1).unwrap())
            }
            18 => {
                let c = ProductTag::generate_random();
                (c.0, serde_json::to_string(&c.1).unwrap())
            }
            19 => {
                let c = ProductTagString::generate_random();
                (c.0, serde_json::to_string(&c.1).unwrap())
            }
            20 => {
                let c = AddParentToChild::generate_random();
                (c.0, serde_json::to_string(&c.1).unwrap())
            }
            21 => {
                let c = ModifyState::generate_random();
                (c.0, serde_json::to_string(&c.1).unwrap())
            }
            22 => {
                let c = ModifyProducedPieces::generate_random();
                (c.0, serde_json::to_string(&c.1).unwrap())
            }
            23 => {
                let c = DeleteShiftById::generate_random();
                (c.0, serde_json::to_string(&c.1).unwrap())
            }
            24 => {
                let c = DeleteShiftByAssetIdAndBeginTimestamp::generate_random();
                (c.0, serde_json::to_string(&c.1).unwrap())
            }
            _ => {
                panic!("Out of range {}", rand_val)
            }
        };
        payloads.push(payload);
    }
    payloads
}

trait GenRandom {
    fn generate_random() -> (String, Self);
}

impl GenRandom for Count {
    fn generate_random() -> (String, Count) {
        let mut rng = rand::thread_rng();
        (
            "count".to_string(),
            Count {
                timestamp_ms: rng.gen(),
                count: rng.gen(),
            },
        )
    }
}

impl GenRandom for ScrapCount {
    fn generate_random() -> (String, ScrapCount) {
        let mut rng = rand::thread_rng();
        (
            "scrapCount".to_string(),
            ScrapCount {
                timestamp_ms: rng.gen(),
                scrap: rng.gen(),
            },
        )
    }
}

impl GenRandom for Barcode {
    fn generate_random() -> (String, Barcode) {
        let mut rng = rand::thread_rng();
        (
            "barcode".to_string(),
            Barcode {
                timestamp_ms: rng.gen(),
                barcode: rng.gen(),
            },
        )
    }
}

impl GenRandom for Activity {
    fn generate_random() -> (String, Activity) {
        let mut rng = rand::thread_rng();
        (
            "activity".to_string(),
            Activity {
                timestamp_ms: rng.gen(),
                activity: rng.gen(),
            },
        )
    }
}

impl GenRandom for DetectedAnomaly {
    fn generate_random() -> (String, DetectedAnomaly) {
        let mut rng = rand::thread_rng();
        (
            "detectedAnomaly".to_string(),
            DetectedAnomaly {
                timestamp_ms: rng.gen(),
                detected_anomaly: generate_random_string(rng.gen_range(5..64)),
            },
        )
    }
}

impl GenRandom for AddShift {
    fn generate_random() -> (String, AddShift) {
        let mut rng = rand::thread_rng();
        (
            "addShift".to_string(),
            AddShift {
                timestamp_ms: rng.gen(),
                timestamp_ms_end: rng.gen(),
            },
        )
    }
}
impl GenRandom for AddOrder {
    fn generate_random() -> (String, AddOrder) {
        let mut rng = rand::thread_rng();
        (
            "addOrder".to_string(),
            AddOrder {
                product_id: generate_random_string(rng.gen_range(5..64)),
                order_id: generate_random_string(rng.gen_range(5..64)),
                target_units: rng.gen(),
            },
        )
    }
}

impl GenRandom for AddProduct {
    fn generate_random() -> (String, AddProduct) {
        let mut rng = rand::thread_rng();
        (
            "addProduct".to_string(),
            AddProduct {
                product_id: rng.gen(),
                time_per_unit_in_seconds: rng.gen(),
            },
        )
    }
}

impl GenRandom for StartOrder {
    fn generate_random() -> (String, StartOrder) {
        let mut rng = rand::thread_rng();
        (
            "startOrder".to_string(),
            StartOrder {
                timestamp_ms: rng.gen(),
                order_id: generate_random_string(rng.gen_range(5..64)),
            },
        )
    }
}

impl GenRandom for EndOrder {
    fn generate_random() -> (String, EndOrder) {
        let mut rng = rand::thread_rng();
        (
            "endOrder".to_string(),
            EndOrder {
                timestamp_ms: rng.gen(),
                order_id: generate_random_string(rng.gen_range(5..64)),
            },
        )
    }
}

impl GenRandom for ProcessValueFloat64 {
    fn generate_random() -> (String, ProcessValueFloat64) {
        let mut rng = rand::thread_rng();
        (
            "processValueFloat64".to_string(),
            ProcessValueFloat64 {
                timestamp_ms: rng.gen(),
                name: generate_random_string(rng.gen_range(5..64)),
                value: rng.gen(),
            },
        )
    }
}

impl GenRandom for ProcessValue {
    fn generate_random() -> (String, ProcessValue) {
        let mut rng = rand::thread_rng();
        (
            "processValue".to_string(),
            ProcessValue {
                timestamp_ms: rng.gen(),
                name: generate_random_string(rng.gen_range(5..64)),
                value: rng.gen(),
            },
        )
    }
}

impl GenRandom for ProcessValueString {
    fn generate_random() -> (String, ProcessValueString) {
        let mut rng = rand::thread_rng();
        (
            "processValueString".to_string(),
            ProcessValueString {
                timestamp_ms: rng.gen(),
                name: generate_random_string(rng.gen_range(5..64)),
                value: generate_random_string(rng.gen_range(5..64)),
            },
        )
    }
}

impl GenRandom for Recommendation {
    fn generate_random() -> (String, Recommendation) {
        let mut rng = rand::thread_rng();
        (
            "recommendation".to_string(),
            Recommendation {
                timestamp_ms: rng.gen(),
                uid: generate_random_string(rng.gen_range(5..64)),
                customer: generate_random_string(rng.gen_range(5..64)),
                location: generate_random_string(rng.gen_range(5..64)),
                asset: generate_random_string(rng.gen_range(5..64)),
                recommendation_type: rng.gen(),
                enabled: rng.gen(),
                recommendation_values: generate_random_string(rng.gen_range(5..64)),
                diagnose_text_de: generate_random_string(rng.gen_range(5..64)),
                diagnose_text_en: generate_random_string(rng.gen_range(5..64)),
                recommendation_text_de: generate_random_string(rng.gen_range(5..64)),
                recommendation_text_en: generate_random_string(rng.gen_range(5..64)),
            },
        )
    }
}

impl GenRandom for State {
    fn generate_random() -> (String, State) {
        let mut rng = rand::thread_rng();
        (
            "state".to_string(),
            State {
                timestamp_ms: rng.gen(),
                state: rng.gen(),
            },
        )
    }
}

impl GenRandom for UniqueProduct {
    fn generate_random() -> (String, UniqueProduct) {
        let mut rng = rand::thread_rng();
        (
            "uniqueProduct".to_string(),
            UniqueProduct {
                begin_timestamp_ms: rng.gen(),
                end_timestamp_ms: rng.gen(),
                product_name: generate_random_string(rng.gen_range(5..64)),
                is_scrap: rng.gen(),
                unique_product_alternative_id: generate_random_string(rng.gen_range(5..64)),
            },
        )
    }
}

impl GenRandom for ScrapUniqueProduct {
    fn generate_random() -> (String, ScrapUniqueProduct) {
        let mut rng = rand::thread_rng();
        (
            "scrapUniqueProduct".to_string(),
            ScrapUniqueProduct {
                uid: generate_random_string(rng.gen_range(5..64)),
            },
        )
    }
}

impl GenRandom for AddMaintenanceActivity {
    fn generate_random() -> (String, AddMaintenanceActivity) {
        let mut rng = rand::thread_rng();
        (
            "addMaintenanceActivity".to_string(),
            AddMaintenanceActivity {
                timestamp_ms: rng.gen(),
                component_name: generate_random_string(rng.gen_range(5..64)),
                activity: rng.gen(),
            },
        )
    }
}

impl GenRandom for ProductTag {
    fn generate_random() -> (String, ProductTag) {
        let mut rng = rand::thread_rng();
        (
            "productTag".to_string(),
            ProductTag {
                timestamp_ms: rng.gen(),
                aid: generate_random_string(rng.gen_range(5..64)),
                name: generate_random_string(rng.gen_range(5..64)),
                value: rng.gen(),
            },
        )
    }
}

impl GenRandom for ProductTagString {
    fn generate_random() -> (String, ProductTagString) {
        let mut rng = rand::thread_rng();
        (
            "productTagString".to_string(),
            ProductTagString {
                timestamp_ms: rng.gen(),
                aid: generate_random_string(rng.gen_range(5..64)),
                name: generate_random_string(rng.gen_range(5..64)),
                value: generate_random_string(rng.gen_range(5..64)),
            },
        )
    }
}

impl GenRandom for AddParentToChild {
    fn generate_random() -> (String, AddParentToChild) {
        let mut rng = rand::thread_rng();
        (
            "addParentToChild".to_string(),
            AddParentToChild {
                timestamp_ms: rng.gen(),
                child_aid: generate_random_string(rng.gen_range(5..64)),
                parent_aid: generate_random_string(rng.gen_range(5..64)),
            },
        )
    }
}

impl GenRandom for ModifyState {
    fn generate_random() -> (String, ModifyState) {
        let mut rng = rand::thread_rng();
        (
            "modifyState".to_string(),
            ModifyState {
                start_time_stamp: rng.gen(),
                end_time_stamp: rng.gen(),
                new_state: rng.gen(),
            },
        )
    }
}
impl GenRandom for ModifyProducedPieces {
    fn generate_random() -> (String, ModifyProducedPieces) {
        let mut rng = rand::thread_rng();
        (
            "modifyProducedPieces".to_string(),
            ModifyProducedPieces {
                count: rng.gen(),
                scrap: rng.gen(),
            },
        )
    }
}
impl GenRandom for DeleteShiftById {
    fn generate_random() -> (String, DeleteShiftById) {
        let mut rng = rand::thread_rng();
        (
            "deleteShiftById".to_string(),
            DeleteShiftById {
                shift_id: rng.gen(),
            },
        )
    }
}

impl GenRandom for DeleteShiftByAssetIdAndBeginTimestamp {
    fn generate_random() -> (String, DeleteShiftByAssetIdAndBeginTimestamp) {
        let mut rng = rand::thread_rng();
        (
            "deleteShiftByAssetIdAndBeginTimestamp".to_string(),
            DeleteShiftByAssetIdAndBeginTimestamp {
                begin_time_stamp: rng.gen(),
            },
        )
    }
}
