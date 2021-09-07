use crate::generator::models::{
    Activity, AddOrder, AddProduct, AddShift, Barcode, Count, DetectedAnomaly, EndOrder,
    ScrapCount, StartOrder,
};

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub(crate) fn generate_random_string(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

pub(crate) fn generate_random(amount: u64) -> Vec<(String, String)> {
    let mut rng = rand::thread_rng();
    let mut payloads = vec![];

    for _ in 0..amount {
        let rand_val = rng.gen_range(0..=9);

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
