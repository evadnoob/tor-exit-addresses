
use iron::prelude::*;
use iron::status;
use router::Router;
use middleware;
use std::sync::{Arc, RwLock};

use persistent::Write;
use iron::typemap::Key;
use downloader;

#[derive(Copy, Clone)]
pub struct HitCounter;

impl Key for HitCounter { type Value = usize; }

pub fn start(lock: Arc<RwLock<downloader::Stuff>>, port: u16) {

    let mut router = Router::new();  

    router.get("/healthcheck", move |req: &mut Request| {
       Ok(Response::with(status::Ok))
    }); 

    
    router.get("/", move |req: &mut Request| {
        info!(" / requesting lock");

        let mutex = req.get::<Write<HitCounter>>().unwrap();
        let mut count = mutex.lock().unwrap();

        let reader = lock.read().unwrap();

        info!(" reader {}", reader.x);
        
        *count += 1;
        info!("Hits: {}", *count);
        
        info!("lock acquired");
        Ok(Response::with((status::Ok, format!("{:?}", reader.buffer))))
    }); 

    let mut chain = Chain::new(router);
    
    chain.link_before(middleware::ResponseTime);
    chain.link_after(middleware::ResponseTime);
    chain.link(Write::<HitCounter>::both(0));

    info!("starting http server, listen port {}", port);
    Iron::new(chain).http(("0.0.0.0", port)).unwrap();
    
}
