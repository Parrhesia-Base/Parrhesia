use enum_dispatch::enum_dispatch;
mod static_data;

use poem::RouteMethod;
pub use static_data::*;

#[enum_dispatch(ServiceType)]
pub trait Service {
    fn extract_endpoint(self) -> RouteMethod;
}

#[enum_dispatch]
pub enum ServiceType {
    StaticDataService,
}

pub enum RequestMethod {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
}

// fn test() {
//     let x: ServiceType = StaticDataService::default().into();
//     x.
// }
