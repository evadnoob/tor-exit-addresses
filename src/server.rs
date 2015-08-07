
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

pub fn start(lock: Arc<RwLock<downloader::Stuff>>) {

    // fn handler(req: &mut Request) -> IronResult<Response> {
        
    //     //Ok(Response::with((status::Ok, format!("Hits: {}", *count))))
    //     //Ok(Response::with((status::Ok, format!("Hits: {:?}", r.len()))))
    //     //Ok(Response::with(status::Ok))
    //     Ok(Response::with((status::Ok, format!("{}", numbers.length()))))
    // }

    let mut router = Router::new();  
    //router.get("/", handler); 
    router.get("/", move |req: &mut Request| {
        info!(" / requesting lock");

        let mutex = req.get::<Write<HitCounter>>().unwrap();
        let mut count = mutex.lock().unwrap();


        let reader = lock.read().unwrap();
        // do some stuff

        info!(" reader {}", reader.x);
        
       
        *count += 1;
        info!("Hits: {}", *count);
            
        //let r = &numbers.lock().unwrap();
        info!("lock acquired");
        //Ok(Response::with((status::Ok, format!("{}", *count))))
        Ok(Response::with((status::Ok, format!("{:?}", reader.buffer))))
    }); 

    let mut chain = Chain::new(router);
 
    chain.link_before(middleware::ResponseTime);
    chain.link_after(middleware::ResponseTime);
     chain.link(Write::<HitCounter>::both(0));
    
    Iron::new(chain).http("localhost:3000").unwrap();
    
}
