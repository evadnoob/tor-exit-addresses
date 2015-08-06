#[macro_use] extern crate log;
extern crate env_logger;
extern crate hyper;
extern crate regex;

mod logging;

use std::io::Read;
use hyper::Client;
use regex::Regex;

///
/// A tiny rust program to downlown tor exit addresses
/// 
fn main() {

    match logging::init() {
        Err(e) => println!("Unable to initialize logging system: {}", e),
        _ => {}
    }

    let mut client = Client::new();
    let url = "https://check.torproject.org/exit-addresses".to_string();
    trace!("url {}", url);
    
    let mut res = client.get(url.trim()) 
        .header(hyper::header::Connection::keep_alive())
        .header(hyper::header::ContentType::json())
        .send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    trace!("res {:?}", res);

    // ExitAddress 46.167.245.172 2015-08-06 21:17:54
    let re = Regex::new(r"^ExitAddress (.*?) .*$").unwrap();
    
    //let re = regex!(r"^ExitAddress");
    // if re.is_match(line));

    for line in body.lines() {
        if re.is_match(line) {
            trace!("{}", line);

            let captures = re.captures(line).unwrap();
            println!("{}", captures.at(1).unwrap());
        }
    }
    
    trace!("done.");
}
