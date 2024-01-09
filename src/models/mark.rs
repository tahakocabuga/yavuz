use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


#[derive(sqlx::FromRow, Deserialize, Serialize, ToSchema)]
pub struct Mark {
    pub id: i64,
    #[schema(example = "Marmara Denizi")]
    pub title: String,
    #[schema(example = "2021-01-01T00:00:00Z")]
    pub time: String,
    #[schema(example = "[https://via.placeholder.com/150, https://via.placeholder.com/150, https://via.placeholder.com/150]")]
    pub images: String,
    #[schema(example = "148.9819")]
    pub latitude: f64,
    #[schema(example = "-35.3981")]
    pub longitude: f64,
}


#[derive(sqlx::FromRow, Deserialize, Serialize, ToSchema)]
pub struct NewMark {
    #[schema(example = "Marmara Denizi")]
    pub title: String,
    #[schema(example = "2021-01-01T00:00:00Z")]
    pub time: String,
    #[schema(example = "[https://via.placeholder.com/150, https://via.placeholder.com/150, https://via.placeholder.com/150]")]
    pub images: String,
    #[schema(example = 148.9819)]
    pub latitude: f64,
    #[schema(example = -35.3981)]
    pub longitude: f64,
}



#[derive(Deserialize, Serialize, sqlx::FromRow, ToSchema)]
pub struct UpdateMark {
    #[schema(example = "Marmara Denizi")]
    pub title: String,
    #[schema(example = "2021-01-01T00:00:00Z")]
    pub time: String,
    #[schema(example = "[https://via.placeholder.com/150, https://via.placeholder.com/150, https://via.placeholder.com/150]")]
    pub images: String, 
    #[schema(example = "148.9819")]
    pub latitude: f64,
    #[schema(example = "-35.3981")]
    pub longitude: f64,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct MarkResponse {
    pub id: i64,
    pub title: String,
    pub time: String,
    pub images: String,
    pub latitude: f64,
    pub longitude: f64,
}

impl Default for MarkResponse {
    fn default() -> Self {
        MarkResponse {
            id: 0,
            title: String::default(),
            time: String::default(),
            images: String::default(),
            latitude: 0.0,
            longitude: 0.0,
        }
    }
}

impl From<Mark> for MarkResponse {
    fn from(mark: Mark) -> Self {
        MarkResponse {
            id: mark.id,
            title: mark.title,
            time: mark.time,
            images: mark.images,
            latitude: mark.latitude,
            longitude: mark.longitude,
        }
    }
}