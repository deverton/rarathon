extern crate hyper;
extern crate rustc_serialize;
extern crate url;

pub use self::client::Client;
pub use self::structs::{MarathonTask, ServiceDefinition};

mod client;
mod structs;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Apps {
    pub apps: Vec<ServiceDefinition>,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Tasks {
    pub tasks: Vec<MarathonTask>,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Leader {
    pub leader: String,
}

