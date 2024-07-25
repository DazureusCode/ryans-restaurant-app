pub mod api;
pub mod db;
pub mod domain;
pub mod protocol;

use crate::db::Storage;

pub struct ServerState {
    pub db: Box<dyn Storage>,
}
