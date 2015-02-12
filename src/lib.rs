#![feature(io)]
extern crate hyper;
extern crate "rustc-serialize" as rustc_serialize;
extern crate url;

pub use client::Client;

mod client;
mod structs;
