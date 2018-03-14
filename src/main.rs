extern crate futures;
extern crate gotham;
#[macro_use]
extern crate gotham_derive;
extern crate hyper;
extern crate mime;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use hyper::{Response, StatusCode};
use hyper::{Get, Head};

use gotham::http::response::create_response;
use gotham::state::State;
use gotham::router::Router;
use gotham::router::builder::*;

mod handlers;
use self::handlers::*;

fn router() -> Router {
    build_simple_router(|route| {
        route.request(vec![Get, Head], "/").to(say_hello);
        route
            .get("/products/:id")
            .with_path_extractor::<handlers::products::PathExtractor>()
            .to(products::index);
        route.post("/products").to(products::create);
    })
}

pub fn say_hello(state: State) -> (State, Response) {
    let msg = String::from("Hi, you've reached the index page");
    let res = create_response(
        &state,
        StatusCode::Ok,
        Some((msg.into_bytes(), mime::TEXT_PLAIN)),
    );
    (state, res)
}

pub fn main() {
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    // TODO: why unwrap the Result? No such thing in the example
    gotham::start(addr, || Ok(router()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use gotham::test::TestServer;
    use hyper::StatusCode;

    #[test]
    fn index_get() {
        let test_server = TestServer::new(router()).unwrap();
        let response = test_server
            .client()
            .get("http://localhost")
            .perform()
            .unwrap();
        assert_eq!(response.status(), StatusCode::Ok);
        let body = response.read_body().unwrap();
        assert_eq!(
            String::from_utf8(body).unwrap(),
            "Hi, you've reached the index page"
        );
    }

    #[test]
    fn get_product_int() {
        let test_server = TestServer::new(router()).unwrap();
        let response = test_server
            .client()
            .get("http://localhost/products/1234")
            .perform()
            .unwrap();
        assert_eq!(response.status(), StatusCode::Ok);
        let body = response.read_body().unwrap();
        assert_eq!(String::from_utf8(body).unwrap(), "Product: 1234");
    }

    #[test]
    fn get_product_wrong_type() {
        let test_server = TestServer::new(router()).unwrap();
        let response = test_server
            .client()
            .get("http://localhost/products/abcd")
            .perform()
            .unwrap();
        assert_eq!(response.status(), StatusCode::BadRequest);
    }
}
