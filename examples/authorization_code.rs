extern crate oauth2_server;
extern crate iron;
extern crate router;
use oauth2_server::code_grant::iron::IronGranter;
use oauth2_server::code_grant::authorizer::Storage;

fn main() {
    let ohandler = IronGranter::new({
        let mut storage = Storage::new();
        storage.register_client("myself", iron::Url::parse("http://localhost:8020/my_endpoint").unwrap());
        storage
    });

    let mut router = router::Router::new();
    router.get("/authorize", ohandler.authorize(), "authorize");
    router.get("/my_endpoint", client, "client");

    iron::Iron::new(router).http("localhost:8020").unwrap();

    fn client(_req: &mut iron::Request) -> iron::IronResult<iron::Response> {
        Ok(iron::Response::with((iron::status::Ok, "Processing oauth request")))
    }
}