use crate::model::product::Product;

use super::get_db_handle;


pub async fn insert_product(product: Product) {
    let handle = get_db_handle().await;
    let results = handle.into();
    return match results() {
        None => panic!("unable to insert objects"),
        Some(value) => Ok(value)
    };
}