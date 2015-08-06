#[macro_use] extern crate log;
extern crate env_logger;
extern crate regex;
extern crate hyper;
extern crate iron;
extern crate router;
extern crate time;


mod logging;
mod middleware;
mod downloader;
mod server;

///
/// A tiny rust program to download tor exit addresses
/// 
fn main() {

    match logging::init() {
        Err(e) => println!("Unable to initialize logging system: {}", e),
        _ => {}
    }

    downloader::start();
    server::start();
  
}
