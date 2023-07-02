use crate::model::product::Product;
use bson::{ oid::ObjectId, Document, doc };
use futures::TryStreamExt;
use mongodb::{ Collection, error::Error };
use super::get_db_handle;

async fn get_handle() -> Collection<Product> {
  return get_db_handle().await.collection::<Product>("products");
}

async fn get_products_with_filter(filter: Option<Document>) -> Result<Vec<Product>, Error> {
  let handle = get_handle().await;
  let results = handle.find(filter, None).await?;
  let vec = results.try_collect::<Vec<Product>>().await?;
  return Ok(vec);
}

pub async fn get_products() -> Result<Vec<Product>, Error> {
  return Ok(get_products_with_filter(None).await?);
}

pub async fn get_product(oid: ObjectId) -> Result<Option<Product>, Error> {
  let handle = get_handle().await;
  let result = handle.find_one(doc! { "_id": oid }, None).await;
  return match result {
    Err(value) => Err(value),
    Ok(value) => Ok(value),
  };
}

pub async fn insert_product(product: Product) -> Result<ObjectId, Error> {
  let handle = get_handle().await;
  let result = handle.insert_one(product, None).await?;
  return match result.inserted_id.as_object_id() {
    None => panic!("Unable to insert object."),
    Some(value) => Ok(value),
  };
}

pub async fn update_product(product: Product) -> Result<(), Error> {
  let update_doc =
    doc! {
    "$set": {
      "_id": product.id,
      "name": product.name,
      "description": product.description,
      "price": product.price,
      "category": product.category,
      "stock": product.stock,
      "tags": product.tags
    }
  };

  let handle = get_handle().await;
  return match handle.update_one(doc! { "_id": product.id }, update_doc, None).await {
    Err(value) => Err(value),
    Ok(_) => Ok(()),
  };
}

pub async fn delete_product(oid: ObjectId) -> Result<(), Error> {
  let handle = get_handle().await;
  return match handle.delete_one(doc! { "_id": oid }, None).await {
    Err(value) => Err(value),
    Ok(_) => Ok(()),
  };
}