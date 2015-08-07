
extern crate hyper;

use std::thread;
use std::sync::{Arc, RwLock};

use std::io::Read;
use hyper::Client;
use regex::Regex;

#[derive(Debug)]
pub struct Stuff {
    pub x: i32,
    pub buffer: Vec<String>
       
}

pub fn start(lock: Arc<RwLock<Stuff>>) {

    match thread::Builder::new().name("tor-exit-address-downloader".to_string()).spawn(move || {

        loop {
            
           info!("starting download of tor exit addresses.");
            
            let mut client = Client::new();
            let url = "https://check.torproject.org/exit-addresses".to_string();
            trace!("url {}", url);
            
            let mut res = client.get(url.trim()) 
                .header(hyper::header::Connection::close())
                .header(hyper::header::ContentType::json())
                .send().unwrap();

            let mut body = String::new();
            res.read_to_string(&mut body).unwrap();
            trace!("res {:?}", res);

            // example of what gets ruturned from that url above:
            // ExitAddress 46.167.245.172 2015-08-06 21:17:54
            let re = Regex::new(r"^ExitAddress (.*?) .*$").unwrap();
            {
                
                let mut writer = lock.write().unwrap();
                
                writer.x += 1;
                writer.buffer.clear();
                
                for line in body.lines() {
                    if re.is_match(line) {
                        trace!("{}", line);

                        let captures = re.captures(line).unwrap();
                        //println!("{}", captures.at(1).unwrap());
                        writer.buffer.push(captures.at(1).unwrap().to_string());
                    }
                }

            }
            
            thread::park_timeout_ms(60000);
            
        }
    }) {
        Ok(_) => info!("download thread started ok"),
        Err(e) => warn!("error {}", e)
    }

}
