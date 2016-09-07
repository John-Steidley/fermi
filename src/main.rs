extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;

use iron::{Iron, Request, Response, IronResult};
use iron::status;
use router::{Router};
use mount::Mount;
use staticfile::Static;

use std::path::Path;

fn main() {
    let mut router = Router::new();
    router.get("/", handler, "handler");
    router.get("/:query", query_handler, "query_handler");

    let mut mount = Mount::new();
    mount
        .mount("/api/v1/", router)
        .mount("/", Static::new(Path::new("assets/")));

    Iron::new(mount).http("127.0.0.1:3000").unwrap();

    fn handler(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "OK, Connor")))
    }

    fn query_handler(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>()
            .unwrap().find("query").unwrap_or("/");
        Ok(Response::with((status::Ok, *query)))
    }
}