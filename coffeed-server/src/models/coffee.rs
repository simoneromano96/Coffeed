use wither_derive::Model;
use serde_derive::{Deserialize, Serialize};

#[derive(Model, Serialize, Deserialize)]
pub struct Coffee {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    #[model(index(index = "asc", unique = "true"))]
    pub name: String,

    pub price: f64,

    pub image_url: String,
    
    pub description: Option<String>,
}