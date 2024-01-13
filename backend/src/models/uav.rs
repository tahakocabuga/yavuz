use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, ToSchema, Clone)]
pub struct UavPosition {
    #[schema(example = 148.9819)]
    pub latitude: f64,
    #[schema(example = -35.3981)]
    pub longitude: f64,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct NewUavPosition {
    #[schema(example = "148.9819")]
    pub latitude: f64,
    #[schema(example = "-35.3981")]
    pub longitude: f64,
}

impl Default for UavPosition {
    fn default() -> Self {
        UavPosition {
            latitude: 0.0,
            longitude: 0.0,
        }
    }
}

pub struct SharedUavPosition {
    position: UavPosition,
}

impl SharedUavPosition {
    pub fn new() -> Self {
        // Initialize with a default position
        Self {
            position: UavPosition {
                latitude: 0.0,
                longitude: 0.0,
            },
        }
    }

    pub fn get_position(&self) -> UavPosition {
        self.position.clone()
    }

    pub fn update_position(&mut self, new_position: UavPosition) {
        self.position = new_position;
    }
}


#[derive(Debug, Deserialize, Serialize)]
pub struct UavPositionResponse {
    pub latitude: f64,
    pub longitude: f64,
}

impl Default for UavPositionResponse {
    fn default() -> Self {
        UavPositionResponse {
            latitude: 0.0,
            longitude: 0.0,
        }
    }
}

impl From<UavPosition> for UavPositionResponse {
    fn from(uav_position: UavPosition) -> Self {
        UavPositionResponse {
            latitude: uav_position.latitude,
            longitude: uav_position.longitude,
        }
    }
}
