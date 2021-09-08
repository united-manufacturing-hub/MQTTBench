use crate::generator::random::{generate_random, generate_random_string};
use crate::{print_message_per_sec, HOST, PORT};
use log::info;

use rumqttc::{AsyncClient, Event, MqttOptions, Outgoing, QoS};
use std::thread;
use std::time::{Duration, Instant};

pub(crate) async fn bench() {
    let name: String = generate_random_string(30);
    let mut mqttoptions = MqttOptions::new(name, HOST, PORT);
    mqttoptions.set_keep_alive(5);
    mqttoptions.set_clean_session(true);
    mqttoptions.set_credentials("TESTING", "");

    let base_topic = "ia/factoryinsight/testLocation/testMachine/";
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client
        .subscribe(base_topic, QoS::AtLeastOnce)
        .await
        .unwrap();

    let n_messages: u64 = 100_000;
    let random_messages = generate_random(n_messages);
    info!(
        "Beginning count bench, with 1 thread and {} messages",
        n_messages
    );
    let start = Instant::now();

    tokio::task::spawn(async move {
        for message in random_messages {
            client
                .publish(
                    format!("{}{}", base_topic, message.0),
                    QoS::ExactlyOnce,
                    false,
                    message.1.as_bytes(),
                )
                .await
                .unwrap();
            //println!("Send: Prefix: {}, Message: {:#?}", message.0, message.1);
            //thread::sleep(Duration::from_secs(1))
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
