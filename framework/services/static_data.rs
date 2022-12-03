use poem::{get, handler, Endpoint, IntoEndpoint, IntoResponse, Response, Route, RouteMethod};

use super::{RequestMethod, Service};

#[handler]
pub fn static_handler() -> String {
    "Hi there!".into()
}

#[derive(Default)]
pub struct StaticDataService {
    pub supported_methods: Vec<RequestMethod>,
    pub data: Response,
}

impl Service for StaticDataService {
    fn extract_endpoint(self) -> RouteMethod {
        // static_handler
        todo!()
    }
}

trait Test {
    // fn test(self) -> dyn poem::Endpoint<Output = dyn IntoResponse>;
    // fn test2(self) -> Box<dyn IntoEndpoint<Endpoint = dyn Endpoint<Output = dyn IntoResponse>>>;
}

enum ServiceTypes {
    Endpoint(dyn Endpoint),
}

// impl<T> Test<T> for StaticDataService
// where
//     T: IntoEndpoint,
// {
//     fn test(self) -> T {
//         get(static_handler)
//     }
// }

// impl Test for StaticDataService {
//     fn test2(self) -> Box<dyn IntoEndpoint<Endpoint = dyn Endpoint<Output = dyn IntoResponse>>> {
//         Box::new(get(static_handler))
//     }
// }

fn test() -> impl Endpoint {
    static_handler
}

fn fun_name() {
    let r = Route::new();
    let r = r.at("/you", static_handler);
    // let r = r.at("/", test());
}
