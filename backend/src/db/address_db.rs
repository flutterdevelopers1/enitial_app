use bson::{ oid::ObjectId, doc };
use futures::TryStreamExt;
use mongodb::{ Collection, error::Error };
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

// get addresses by id 
pub async fn get_address_by_id(address_id: &ObjectId) -> Result<Option<Address>, Error> {
  let handle = get_handle().await;
  let result = handle.find_one(doc! { "_id": address_id }, None).await?;
  return Ok(result);
}

// deleete address 
pub async fn delete_address_by_id(address_id: &ObjectId) -> Result<(), Error> {
  let handle = get_handle().await;
  let _result = handle.delete_one(doc! { "_id": address_id }, None).await?;
  return Ok(());
}

// insert a new address
pub async fn insert_address(address: &Address) -> Result<ObjectId, Error> {
  let handle = get_handle().await;
  let result = handle.insert_one(address, None).await?;
  return Ok(result.inserted_id.as_object_id().expect("Unable to insert user!"));
}

// update address
pub async fn update_address(address: Address) -> Result<(), Error> {
  let doc =
    doc! {
    "$set": {
      "userId": address.user_id,
      "addressType": address.address_type,
      "fullName": address.full_name,
      "address1": address.address1,
      "address2": address.address2,
      "city": address.city,
      "county": address.county,
      "state": address.state,
      "country": address.country,
      "postalCode": address.postal_code
    }
  };

  let handle = get_handle().await;
  let _result = handle.update_one(doc! { "_id": &address.id }, doc, None).await?;
  return Ok(());
}
