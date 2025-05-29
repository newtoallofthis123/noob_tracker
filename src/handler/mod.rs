use crate::db::Db;

mod user;
mod account;

pub struct Handler {
    db: Db,
}

impl Handler {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}
