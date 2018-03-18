#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate serde_derive;

pub mod db;
pub use db::schema;
pub use db::models;
pub use db::utils;
