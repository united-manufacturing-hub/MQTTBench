use crate::generator::random::{generate_random, generate_random_image, generate_random_string};
use crate::{print_message_per_sec};
use log::info;

use rumqttc::{AsyncClient, ConnectionError, Event, MqttOptions, Outgoing, QoS};

use std::time::Instant;
use crate::generator::models::ProductImage;

pub(crate) async fn bench(host: String, port: u16) {
    let name: String = generate_random_string(30);
    let mut mqttoptions = MqttOptions::new(name, host, port);
    mqttoptions.set_keep_alive(5);
    mqttoptions.set_clean_session(true);
    mqttoptions.set_credentials("TESTING", "");

    let base_topic = "ia/factoryinsight/2/3/";
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client
        .subscribe(base_topic, QoS::AtLeastOnce)
        .await
        .unwrap();

    let start = Instant::now();
    let n_messages = 100_000;

    tokio::task::spawn(async move {
        for i in 0..n_messages {
            let message = generate_random_image();
            client
                .publish(
                    format!("{}{}", base_topic, message.0),
                    QoS::ExactlyOnce,
                    false,
                    message.1.as_bytes(),
                )
                .await
                .unwrap();
        }
    });

    let mut published: u64 = 0;
    loop {
        let notification = eventloop.poll().await;

        match notification {
            Ok(n) => {
                match n {
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
            }
            Err(e) => {
                println!("{:?}", e)
            }
        }


        if published == n_messages {
            break;
        }
    }
    print_message_per_sec(published, n_messages, start);
}
