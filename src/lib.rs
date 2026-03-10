#[macro_use]
extern crate diesel;

mod adapter;
mod error;

mod models;
mod schema;

mod actions;

pub use casbin;

pub use adapter::DieselAdapter;
pub use actions::Connection;
pub use error::Error;
