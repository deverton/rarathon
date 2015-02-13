#![feature(collections, env)]
extern crate getopts;
extern crate rarathon;
extern crate "rustc-serialize" as rustc_serialize;

use getopts::{Options, ParsingStyle};
use rustc_serialize::json;
use std::env;

fn leader(client: rarathon::Client) {
    println!("{}", json::encode(&client.leader()).unwrap());
}

fn list_applications(client: rarathon::Client) {
    println!("Would list");
}

fn list_tasks(client: rarathon::Client, args: Vec<String>) {
    let mut opts = Options::new();
    opts.reqopt("i", "id", "A unique identifier for the app.", "");

    let matches = match opts.parse(args) {
        Ok(m)  => m,
        Err(f) => panic!(f.to_string()),
    };

    let id = matches.opt_str("id").expect("id specified");

    println!("{}", json::encode(&client.list_tasks(id)).unwrap());
}

fn print_usage(msg: &str, opts: Options) {
    let brief = "Usage: rarathon [global options] [command] [options]";
    env::set_exit_status(-1);
    println!("{}\n{}", msg, opts.usage(brief.as_slice()));
}

fn main() {
    let mut opts = Options::new();
    opts.parsing_style(ParsingStyle::StopAtFirstFree);
    opts.optopt("H", "host", "Marathon host (default http://localhost:8080, or MARATHON_HOST)", "URL");
    opts.optopt("U", "username", "Username to authenticate against Marathon (optional).", "USERNAME");
    opts.optopt("P", "password", "Password to authenticate against Marathon (optional).", "PASSWORD");

    let mut args = env::args().collect::<Vec<_>>();

    let program = args.remove(0);

    let matches = match opts.parse(args) {
        Ok(m)  => m,
        Err(f) => panic!(f.to_string()),
    };

    let host = matches.opt_str("host").unwrap_or(String::from_str("http://localhost:8080"));
    let username = matches.opt_str("username");
    let password = matches.opt_str("password");

    let client = rarathon::Client::new(host, username, password);

    let mut free_args = matches.free.clone();

    if free_args.is_empty() {
        print_usage("No command specified", opts);
        return;
    }

    let command = free_args.remove(0);

    match &command[] {
        "leader"     => leader(client),
        "list"       => list_applications(client),
        "list_tasks" => list_tasks(client, matches.free),
        command      => print_usage(&format!("Unknown command {}\n", command)[], opts),
    }

}

