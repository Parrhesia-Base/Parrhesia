// use poem::{get, handler, Endpoint, IntoEndpoint, IntoResponse, Response, Route, RouteMethod};

use super::Service;

// #[handler]
// pub fn static_handler() -> String {
//     "Hi there!".into()
// }

async fn static_handler() -> impl IntoResponse {
    "Hello there!"
}

use axum::{
    body::{Body, HttpBody},
    handler::Handler,
    response::IntoResponse,
    routing::{get, on, MethodFilter, MethodRouter},
    Router,
};

/// Bring-Your-Own-Handler Service
pub struct ByohService {
    // pub supported_methods: Vec<RequestMethod>,
    // =pub data: Response,
    pub supported_methods: MethodFilter,
    service: MethodRouter,
    // handler: dyn Handler<, ()>,
}

impl Service for ByohService {
    fn extract_service(&mut self) -> MethodRouter {
        // static_handler
        // self.handler.
        // let r = Router::new().route("path", get(static_handler));
        // on(self.supported_methods, r)
        std::mem::take(&mut self.service)
    }
}

// impl Default for ByohService {
//     fn default() -> Self {
//         Self {
//             // handler: static_handler,
//             supported_methods: MethodFilter::empty(),
//         }
//     }
// }

impl ByohService {
    pub fn set_handler<H, T>(&mut self, handler: H)
    where
        H: Handler<T, (), Body>,
        T: 'static,
    {
        self.service = on(self.supported_methods, handler);
    }
}

// trait Test {
//     // fn test(self) -> dyn poem::Endpoint<Output = dyn IntoResponse>;
//     // fn test2(self) -> Box<dyn IntoEndpoint<Endpoint = dyn Endpoint<Output = dyn IntoResponse>>>;
//     fn get_method_router() -> MethodRouter;
// }

// impl Test for StaticDataService {
//     fn get_method_router() -> MethodRouter {
//         get(string_handler)
//     }
// }

// enum ServiceTypes {
//     Endpoint(dyn Endpoint),
// }

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

// fn test() -> impl Endpoint {
//     static_handler
// }

// fn fun_name() {
//     let r = Route::new();
//     let r = r.at("/you", static_handler);
//     // let r = r.at("/", test());
// }
