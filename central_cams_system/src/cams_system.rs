use std::io::Error;

use central_cams_system::cams_system_config::CamSystemConfig;
use logger::logger_handler::Logger;
use mqtt::client::mqtt_client::MqttClient;
use shared::models::{
    cam_model::{
        cam::{Cam, CamState},
        cam_list::CamList,
    },
    inc_model::incident::Incident,
};
use walkers::Position;

pub struct CamsSystem {
    pub system: CamList,
    pub config: CamSystemConfig,
}

impl CamsSystem {
    pub fn init(path: String) -> Result<Self, Error> {
        let config = CamSystemConfig::from_file(path)?;

        let system = CamList::init(&config.db_path);

        Ok(CamsSystem { system, config })
    }

    pub fn add_new_camara(&mut self, cam: Cam) -> Cam {
        let new_cam = cam.clone();
        self.system.cams.insert(cam.id, cam);
        new_cam
    }

    pub fn delete_camara(&mut self, id: &u8) -> Result<Cam, Error> {
        let cam = self.system.cams.get(id).unwrap();
        if cam.state == CamState::Alert {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "ERROR - No se puede eliminar una cámara en modo alerta",
            ));
        }
        let cam = self.system.cams.remove(id).unwrap();
        Ok(cam)
    }

    pub fn modify_cam_position(&mut self, id: &u8, new_pos: Position) -> Result<Cam, Error> {
        if let Some(cam) = self.system.cams.get_mut(id) {
            if cam.state == CamState::Alert {
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    "ERROR - No se puede modificar la posición de una cámara en modo alerta",
                ));
            }
            cam.location = new_pos;
            return Ok(cam.clone());
        }
        Err(Error::new(
            std::io::ErrorKind::Other,
            "No se encontró la cámara",
        ))
    }

    pub fn modify_cameras_state(
        &mut self,
        incident_location: Position,
        new_state: CamState,
    ) -> Vec<Cam> {
        let mut modified_cams = Vec::new();

        for cam in self.system.cams.values_mut() {
            if (incident_location.lat() - cam.location.lat()).abs() < self.config.range_alert
                && (incident_location.lon() - cam.location.lon()).abs() < self.config.range_alert
            {
                match new_state {
                    CamState::Alert => {
                        cam.state = new_state.clone();
                        cam.incidents_covering += 1;
                    }
                    CamState::SavingEnergy => {
                        if cam.incidents_covering == 0 {
                            continue;
                        }
                        cam.incidents_covering -= 1;
                        if cam.incidents_covering == 0 {
                            cam.state = new_state.clone();
                        }
                    }
                    _ => {}
                }
                modified_cams.push(cam.clone());
            }
        }

        let mut close_cameras_modified = Vec::new();

        for cam in self.system.cams.values_mut() {
            for modified_cam in &modified_cams {
                if (modified_cam.location.lat() - cam.location.lat()).abs()
                    < self.config.range_alert_between_cameras
                    && (modified_cam.location.lon() - cam.location.lon()).abs()
                        < self.config.range_alert_between_cameras
                {
                    if !modified_cams.contains(cam) && new_state == CamState::Alert {
                        cam.incidents_covering += 1;
                        cam.state = new_state.clone();
                        close_cameras_modified.push(cam.clone());
                    }

                    if !modified_cams.contains(cam) && new_state == CamState::SavingEnergy {
                        if cam.incidents_covering == 0 {
                            continue;
                        }
                        cam.incidents_covering -= 1;
                        if cam.incidents_covering == 0 {
                            cam.state = new_state.clone();
                            close_cameras_modified.push(cam.clone());
                        }
                    }
                    break;
                }
            }
        }
        for cam in close_cameras_modified {
            modified_cams.push(cam);
        }

        modified_cams
    }

    pub fn list_cameras(&self) {
        if self.system.cams.is_empty() {
            println!("  No hay cámaras registradas");
            return;
        }
        println!("{}", self.system);
    }

    pub fn process_incident_in_progress(
        &mut self,
        client: &mut MqttClient,
        incident: Incident,
        logger: &Logger,
    ) {
        let modified_cams = self.modify_cameras_state(incident.location, CamState::Alert);

        self.system.save(&self.config.db_path).unwrap();

        for cam in modified_cams {
            match client.publish(cam.as_bytes(), "camaras".to_string(), logger) {
                Ok(_) => {
                    println!("Modifica estado de la cámara en modo alerta");
                }
                Err(e) => {
                    println!("Error al publicar mensaje: {}", e);
                }
            }
        }
        self.list_cameras();
    }

    pub fn process_incident_resolved(
        &mut self,
        client: &mut MqttClient,
        incident: Incident,
        logger: &Logger,
    ) {
        let modified_cams = self.modify_cameras_state(incident.location, CamState::SavingEnergy);

        self.system.save(&self.config.db_path).unwrap();

        for cam in modified_cams {
            match client.publish(cam.as_bytes(), "camaras".to_string(), logger) {
                Ok(_) => {
                    println!("Modifica estado de la cámara en modo ahorro de energía");
                }
                Err(e) => {
                    println!("Error al publicar mensaje: {}", e);
                }
            }
        }
        self.list_cameras();
    }
}
