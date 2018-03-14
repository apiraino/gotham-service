extern crate gotham;
extern crate hyper;
use handlers::gotham::http::response::create_response;
use self::hyper::{Response, StatusCode};
use gotham::state::State;
extern crate mime;

pub mod products {
    use gotham::state::FromState;

    #[derive(Deserialize, StateData, StaticResponseExtender)]
    pub struct PathExtractor {
        id: i32,
    }

    use super::*;
    pub fn index(state: State) -> (State, Response) {
        let res = {
            let product = PathExtractor::borrow_from(&state);
            println!("requested id: {}", product.id);
            create_response(
                &state,
                StatusCode::Ok,
                Some((
                    format!("Product: {}", product.id).into_bytes(),
                    mime::TEXT_PLAIN,
                )),
            )
        };
        (state, res)
    }

    pub fn create(state: State) -> (State, Response) {
        let res = create_response(&state, StatusCode::Ok, None);
        println!("created item!");
        (state, res)
    }

}
