use std::io::Error;

use logger::logger_handler::create_logger_handler;
use monitoring_app::{app::MonitoringApp, app_config::MonitoringAppConfig};
use mqtt::{
    client::mqtt_client::MqttClient,
    config::{client_config::ClientConfig, mqtt_config::Config},
};

fn main() -> Result<(), Error> {
    let client_config_path = "monitoring_app/config/client_config.txt";
    let app_config_path = "monitoring_app/config/app_config.txt";

    let config = ClientConfig::from_file(String::from(client_config_path))?;

    let log_path = config.general.log_path.to_string();
    let logger_handler = create_logger_handler(&log_path)?;
    let logger = logger_handler.get_logger();

    let app_config = MonitoringAppConfig::new(String::from(app_config_path))?;

    let client = match MqttClient::init(config) {
        Ok(r) => r,
        Err(e) => {
            logger.close();
            logger_handler.close();
            return Err(e);
        }
    };

    let handlers = match MonitoringApp::init(client, logger.clone(), app_config) {
        Ok(r) => r,
        Err(e) => {
            logger.close();
            logger_handler.close();
            return Err(Error::new(std::io::ErrorKind::Other, e));
        }
    };

    logger.close();
    logger_handler.close();
    handlers.broker_listener.join().unwrap()?;
    handlers.message_handler.join().unwrap();
    Ok(())
}
