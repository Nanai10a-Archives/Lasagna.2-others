pub mod impls;

use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use crate::types;

#[derive(Serialize, Deserialize)]
pub struct SchemaForSerde {
    pub user: String,
    pub max_num: u32,
    pub posts: SmallVec<[PostForSerde; 1024]>,
}

pub struct Schema {
    pub user: String,
    pub max_num: u32,
    pub posts: smallvec::SmallVec<[Post; 1024]>,
}

#[derive(Debug, Clone)]
pub struct Post {
    pub num: u32,
    pub content: String,
    pub created: types::Date,
    pub updated: Option<types::Date>,
    pub is_deleted: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct PostForSerde {
    pub num: u32,
    pub content: String,
    pub created: String,
    pub updated: Option<String>,
    pub is_deleted: Option<bool>,
}
