use rdkafka::{
    producer::{BaseProducer, BaseRecord, Producer},
    ClientConfig,
};
use std::time::Duration;

fn main() {
    let producer: BaseProducer = ClientConfig::new()
        .set("bootstrap.servers", "host.docker.internal:9094")
        .create()
        .expect("Could not create producer!");

    let topic = "test";
    producer
        .send(BaseRecord::<String, str>::to(topic).payload("Test from rust"))
        .unwrap_or_else(|e| {
            eprintln!("ERROR: {:?}", e);
            std::process::exit(1);
        });

    producer.flush(Duration::from_secs(1)).unwrap_or_else(|e| {
        eprintln!("ERROR: {:?}", e);
        std::process::exit(1);
    });
}
