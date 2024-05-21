use std::{
    io::Error,
    sync::mpsc::Receiver,
    thread::{self, JoinHandle},
};

use mqtt::{
    client::mqtt_client::{MqttClient, MqttClientMessage},
    config::{client_config::ClientConfig, mqtt_config::Config},
};

fn process_messages(receiver: Receiver<MqttClientMessage>) -> Result<JoinHandle<()>, Error> {
    let handler = thread::spawn(move || loop {
        let message_received = receiver.recv().unwrap();
        match message_received.topic.as_str() {
            "cams" => {
                println!(
                    "Mensaje recibido y procesado del topic 'cams': {}",
                    message_received.data
                );
            }
            "dron" => {
                // cambiar estado
            }
            _ => {}
        }
        // leer el mensaje recibido y cambiar estados según corresponda
    });

    Ok(handler)
}

fn main() -> Result<(), Error> {
    let config_path = "central_cams_system/config/cams_config.txt";

    let config = ClientConfig::from_file(String::from(config_path))?;

    let mut client = MqttClient::init(config)?;

    let listener = client.run_listener()?;

    let process_message_handler = process_messages(listener.receiver)?;

    client.subscribe(vec!["cams", "dron"], 1, false, false, 0)?;

    //client.publish("mensaje del cliente".to_string(), "cams".to_string())?;

    listener.handler.join().unwrap()?;
    process_message_handler.join().unwrap();

    Ok(())
}