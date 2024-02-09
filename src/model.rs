use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub username: String,
}

pub type UsersDb = Arc<RwLock<HashMap<Uuid, User>>>;
