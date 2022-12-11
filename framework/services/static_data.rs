// use poem::{get, handler, Endpoint, IntoEndpoint, IntoResponse, Response, Route, RouteMethod};

use super::Service;

// #[handler]
// pub fn static_handler() -> String {
//     "Hi there!".into()
// }

async fn static_handler() -> impl IntoResponse {
    "Hello there!"
}

use anyhow::Result;
use axum::{
    response::IntoResponse,
    routing::{on, MethodFilter, MethodRouter},
};

pub struct StaticDataService {
    // pub supported_methods: Vec<RequestMethod>,
    // =pub data: Response,
    pub supported_methods: MethodFilter,
}

impl Service for StaticDataService {
    fn extract_service(&mut self) -> Result<MethodRouter> {
        // static_handler
        Ok(on(self.supported_methods, static_handler))
    }
}

impl Default for StaticDataService {
    fn default() -> Self {
        Self {
            supported_methods: MethodFilter::empty(),
        }
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
