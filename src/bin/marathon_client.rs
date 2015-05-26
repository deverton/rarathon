extern crate getopts;
extern crate rarathon;
extern crate rustc_serialize;

use getopts::{Options, ParsingStyle};
use rustc_serialize::json;
use std::env;

fn leader(client: rarathon::Client) {
    let result = client.leader();
    println!("{}", result.leader);
}

fn list_apps(client: rarathon::Client) {
    let apps = client.list_apps().apps;
    if apps.is_empty() {
        println!("{}", "No apps currently running.");
    } else {
        for app in apps {
            println!("App ID:     {}", app.id);
            println!("Command:    {}", app.cmd.unwrap_or(String::new()));
            println!("Instances:  {}", app.instances);
            println!("CPUs:       {}", app.cpus);
            println!("Memory:     {}", app.mem);
            for uri in app.uris {
                println!("URI:        {}", uri);
            };
            for (k, v) in app.env {
                println!("ENV:        {}={}", k, v);
            };
            for constraint in app.constraints {
                println!("Constraint: {}", json::encode(&constraint).unwrap());
            };
            println!("");
        }
    }
}

fn list_tasks(client: rarathon::Client, args: Vec<String>) {
    let mut opts = Options::new();
    opts.reqopt("i", "id", "A unique identifier for the app.", "");

    let matches = match opts.parse(args) {
        Ok(m)  => m,
        Err(f) => panic!(f.to_string()),
    };

    let id = matches.opt_str("id").expect("id specified");

    let tasks = client.list_tasks(id);

    println!("{}", json::encode(&tasks).unwrap());
}

fn print_usage(program: &str, msg: &str, opts: Options) {
    let brief = format!("Usage: {} [global options] [command] [options]", program);
    println!("{}\n{}", msg, opts.usage(&brief));
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

    let host = matches.opt_str("host").unwrap_or("http://localhost:8080".to_string());
    let username = matches.opt_str("username");
    let password = matches.opt_str("password");

    let client = rarathon::Client::new(host, username, password);

    let mut free_args = matches.free.clone();

    if free_args.is_empty() {
        return print_usage(&program, "No command specified", opts);
    }

    let command = free_args.remove(0);

    match &command[..] {
        "leader"     => leader(client),
        "list"       => list_apps(client),
        "list_tasks" => list_tasks(client, matches.free),
        command      => print_usage(&program, &format!("Unknown command {}\n", command)[..], opts),
    }
}

