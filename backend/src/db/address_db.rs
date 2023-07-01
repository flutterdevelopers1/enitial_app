use bson::{oid::ObjectId, doc};
use futures::TryStreamExt;
use mongodb::{Collection, error::Error};
use crate::model::address::Address;
use super::get_db_handle;

async fn get_handle() -> Collection<Address> {
    return get_db_handle().await.collection::<Address>("adresses");
}

// get user address
pub async fn get_addresses_from_user(user_id: &ObjectId) -> Result<Vec<Address>, Error> {
    let handle = get_handle().await;
    let results = handle.find(doc! { "userId": user_id }, None).await?;
    let vec = results.try_collect::<Vec<Address>>().await?;
    return Ok(vec);
}