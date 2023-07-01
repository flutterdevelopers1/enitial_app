use bson::{oid::ObjectId, doc};
use futures::TryStreamExt;
use mongodb::{Collection, error::Error};
use crate::model::address::Address;
use super::get_db_handle;