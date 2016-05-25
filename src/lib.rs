//! Sporm is a simple ORM implemented in rust.
#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]


// extern crate rustc_serialize;
extern crate serde;
extern crate serde_json;
extern crate uuid;
extern crate chrono;
extern crate regex;
extern crate url;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate postgres;
#[cfg(feature = "sqlite")]
extern crate r2d2_sqlite;
#[cfg(feature = "sqlite")]
extern crate rusqlite;
#[cfg(feature = "mysql")]
extern crate mysql;
extern crate time;


pub mod em;
pub mod query;
pub mod dao;
pub mod database;
pub mod platform;
pub mod table;
pub mod writer;
pub mod config;
pub mod pool;
