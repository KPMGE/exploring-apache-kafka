use rdkafka::{producer::{Producer, BaseProducer, BaseRecord}, ClientConfig};
use std::time::Duration;

fn main() {
    let producer: BaseProducer = ClientConfig::new()
        .set("bootstrap.servers", "127.0.0.1:9094")
        .create()
        .expect("Could not create producer!");

    let _ = producer.send(
        BaseRecord::<String, str>::to("test")
            .payload("Test from rust")
    ).map_err(|e| {
        eprintln!("ERROR: could not send the message!");
        panic!("{:?}", e);
    });

    let _ = producer.flush(Duration::from_secs(1));
}
