use rdkafka::{
    producer::{FutureProducer, FutureRecord},
    ClientConfig,
};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "host.docker.internal:9094")
        .create()
        .expect("Could not create producer!");

    let topic = "test";
    producer
        .send(
            FutureRecord::<String, str>::to(topic).payload("Test from rust"),
            Duration::from_secs(2),
        )
        .await
        .map(|(partition, offset)| {
            println!(
                "message sent to partition: {} with offset: {}",
                partition, offset
            );
        })
        .map_err(|(kafka_err, _)| {
            eprintln!("ERROR: {:?}", kafka_err);
        })
        .expect("Could not send the message!");
}
