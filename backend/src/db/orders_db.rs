use bson::{ doc, oid::ObjectId };
use mongodb::{ Collection, error::Error };
use futures::TryStreamExt;
use crate::model::order::Order;
use super::get_db_handle;

async fn get_handle() -> Collection<Order> {
  return get_db_handle().await.collection::<Order>("orders");
}

pub async fn get_orders_by_user_id(user_id: &ObjectId) -> Result<Vec<Order>, Error> {
  let handle = get_handle().await;
  let result = handle.find(doc! { "userId": user_id }, None).await?;
  let vec = result.try_collect::<Vec<Order>>().await?;
  return Ok(vec);
}

pub async fn get_order_by_id(order_id: &ObjectId) -> Result<Option<Order>, Error> {
  let handle = get_handle().await;
  let result = handle.find_one(doc! { "_id": order_id }, None).await?;
  return Ok(result);
}

pub async fn update_order(order: Order) -> Result<(), Error> {
  let doc =
    doc! {
    "$set": {
      "userId": order.user_id,
      "products": order.products,
      "billingAddress": order.billing_address,
      "mailingAddress": order.mailing_address,
      "subtotal": order.subtotal,
      "shippingCost": order.shipping_cost,
      "total": order.total,
      "tax": order.tax,
      "taxState": order.tax_state,
    }
  };

  let handle = get_handle().await;
  let _result = handle.update_one(doc! { "_id": order.id }, doc, None).await?;
  return Ok(());
}

pub async fn delete_order(id: &ObjectId) -> Result<(), Error> {
  let handle = get_handle().await;
  let result = handle.delete_one(doc! { "_id": id }, None).await;
  return match result {
    Err(value) => Err(value),
    Ok(_) => Ok(()),
  };
}

pub async fn insert_order(order: &Order) -> Result<ObjectId, Error> {
  let handle = get_handle().await;
  let result = handle.insert_one(order, None).await?;
  let oid = result.inserted_id.as_object_id();
  return match oid {
    Some(oid) => Ok(oid),
    None => panic!("Unable to insert object."),
  };
}
