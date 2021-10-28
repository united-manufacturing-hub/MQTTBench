#![feature(thread_id_value)]

use crate::bench::single_thread;
use log::{info, LevelFilter};

use std::thread;
use std::time::{Duration, Instant};
use std::str::FromStr;

pub(crate) mod bench;
pub(crate) mod generator;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    simple_logger::SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .with_module_level("MQTTBench", LevelFilter::Debug)
        .init()
        .unwrap();

    let host = std::env::var("HOST").unwrap();
    let port = u16::from_str(&std::env::var("PORT").unwrap()).unwrap();

    loop {
        single_thread::image::bench(host.clone(), port).await;
    }
    /*
loop {
    single_thread::count::bench(host.clone(), port).await;
    cool_down();
    single_thread::random::bench(host.clone(), port).await;
    cool_down();
}

     */
}

/// Waits 10 secs between tests
fn cool_down() {
    info!("Cooling down....");
    let cooldown_time = 10;
    for i in 0..cooldown_time {
        info!("{}/{}", i, cooldown_time);
        thread::sleep(Duration::from_secs(1));
    }
}

/// Print messages per secs
fn print_message_per_sec(messages_send: u64, messages_max: u64, start: Instant) {
    let elapsed = start.elapsed();
    let messages_per_sec = messages_send as f64 / elapsed.as_secs_f64();
    info!(
        "{}/{} in {:?} ({} msg/s)",
        messages_send, messages_max, elapsed, messages_per_sec
    )
}
