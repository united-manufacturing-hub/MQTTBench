use crate::generator::random::generate_random_string;
use crate::{print_message_per_sec, HOST, PORT};
use log::info;

use rumqttc::{AsyncClient, Event, MqttOptions, Outgoing, QoS};
use std::time::Instant;

/// Sends well-formatted "count" mqtt messages
pub(crate) async fn bench() {
    let name: String = generate_random_string(5);
    let mut mqttoptions = MqttOptions::new(name, HOST, PORT);
    mqttoptions.set_keep_alive(5);
    mqttoptions.set_clean_session(true);
    mqttoptions.set_credentials("TESTING", "");

    let topic = "ia/factoryinsight/testLocation/testMachine/count";
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe(topic, QoS::AtLeastOnce).await.unwrap();

    let n_messages: u64 = 100_000;
    info!(
        "Beginning count bench, with 1 thread and {} messages",
        n_messages
    );
    let start = Instant::now();

    tokio::task::spawn(async move {
        for i in 0..=n_messages {
            client
                .publish(
                    topic,
                    QoS::AtLeastOnce,
                    false,
                    format!("{{\"timestamp_ms\": {}, \"count\": {}, \"scrap\": 0}}", i, i).as_bytes(),
                )
                .await
                .unwrap();
        }
    });

    let mut published: u64 = 0;
    loop {
        let notification = eventloop.poll().await.unwrap();
        match notification {
            Event::Incoming(_) => {}
            Event::Outgoing(o) => {
                if let Outgoing::Publish(_) = &o {
                    published += 1;
                    if published % 1000 == 0 {
                        print_message_per_sec(published, n_messages, start);
                    }
                }
            }
        }
        if published == n_messages {
            break;
        }
    }
    print_message_per_sec(published, n_messages, start);
}
