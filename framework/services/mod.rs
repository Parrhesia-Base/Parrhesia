use anyhow::Result;
use axum::routing::MethodRouter;
use enum_dispatch::enum_dispatch;
mod byoh;
mod static_data;

pub use byoh::ByohService;
pub use static_data::StaticDataService;

// use poem::RouteMethod;

pub use axum::routing::MethodFilter;

#[enum_dispatch(ServiceType)]
pub trait Service {
    fn extract_service(&mut self) -> Result<MethodRouter>;
}

#[enum_dispatch]
pub enum ServiceType {
    StaticDataService,
    ByohService,
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
