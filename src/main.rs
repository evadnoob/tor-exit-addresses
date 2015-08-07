#[macro_use] extern crate log;
extern crate env_logger;
extern crate regex;
extern crate hyper;
extern crate iron;
extern crate router;
extern crate time;
extern crate persistent;
extern crate docopt;

mod logging;
mod middleware;
mod downloader;
mod server;
mod cliargs;

use std::sync::{Arc, RwLock};

///
/// A tiny rust program to download tor exit addresses
/// 
fn main() {

    match logging::init() {
        Err(e) => println!("Unable to initialize logging system: {}", e),
        _ => {}
    }

    let args = cliargs::parse();
    info!("args: {:?}", args); 
    info!("arg vector: {:?}", args.get_vec("<args>"));

    let stuff = downloader::Stuff{x: 5, buffer: Vec::new()};
    let rwlock = RwLock::new(stuff);
    let arc = Arc::new(rwlock);
    let local_arc = arc.clone();
    let my_rwlock = arc.clone();

    let port = args.get_str("--port");
    downloader::start(local_arc);
    server::start(my_rwlock, port.parse::<u16>().unwrap());
}
