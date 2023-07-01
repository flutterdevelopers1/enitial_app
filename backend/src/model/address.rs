use bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    // id 
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    // userId
    #[serde(rename = "userId")]
    pub user_id: Option<ObjectId>,

    // address type
    #[serde(rename = "addressType")]
    pub address_type: Option<String>,

    // Details[full name, address1, address2, city, county, state]
    #[serde(rename = "fullName")]
    pub full_name: Option<String>,
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub city: Option<String>,
    pub county: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,

    #[serde(rename = "postalCode")]
    pub postal_code: Option<String>,
}