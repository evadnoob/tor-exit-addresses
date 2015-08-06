use iron::prelude::*;
use iron::status;
use router::Router;
use middleware;

const MAX_BODY_LENGTH: usize = 1024 * 1024 * 10;

pub fn start() {

    fn handler(req: &mut Request) -> IronResult<Response> {
        
        let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
        debug!("query {}", query);
        Ok(Response::with(status::Ok))
    }

    let mut router = Router::new();  
    router.get("/", handler); 

    let mut chain = Chain::new(router);
    //chain.link_before(Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));
 
    chain.link_before(middleware::ResponseTime);
    chain.link_after(middleware::ResponseTime);


    Iron::new(chain).http("localhost:3000").unwrap();
    
}
