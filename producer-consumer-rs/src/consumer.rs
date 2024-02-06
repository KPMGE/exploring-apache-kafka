use rdkafka::{consumer::BaseConsumer, consumer::Consumer, ClientConfig, Message};

#[tokio::main]
async fn main() {
    let consumer: BaseConsumer = ClientConfig::new()
        .set("bootstrap.servers", "host.docker.internal:9094")
        .set("client.id", "consumer-id3")
        .set("group.id", "consumer-group")
        .create()
        .expect("Could not create consumer!");

    let topic = "test";

    consumer.subscribe(&[topic]).unwrap_or_else(|e| {
        eprintln!("ERROR: (could not subscribe to topic):\n {:?}", e);
    });

    for message in consumer.iter() {
        match message {
            Ok(data) => {
                let payload = std::str::from_utf8(data.payload().unwrap()).unwrap();
                let partition = data.partition();
                let offset = data.offset();

                println!("payload: {}", payload);
                println!("partition: {}", partition);
                println!("offset: {}\n", offset);
            }
            Err(e) => println!("ERROR: {:?}", e),
        }
    }
}
