use axum::routing::MethodRouter;
use enum_dispatch::enum_dispatch;
// mod static_data;
mod byoh;

// use poem::RouteMethod;
// pub use static_data::*;

pub use axum::routing::MethodFilter;

#[enum_dispatch(ServiceType)]
pub trait Service {
    fn extract_service(&mut self) -> MethodRouter;
}

#[enum_dispatch]
pub enum ServiceType {
    // StaticDataService,
}

// pub enum RequestMethod {
//     Get,
//     Head,
//     Post,
//     Put,
//     Delete,
//     Connect,
//     Options,
//     Trace,
//     Patch,
// }

// // fn test() {
// //     let x: ServiceType = StaticDataService::default().into();
// //     x.
// // }
