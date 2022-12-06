use std::convert::Infallible;
use std::marker::PhantomData;

use axum::body::HttpBody;
use axum::{
    body::Body,
    handler::Handler,
    routing::{on, MethodFilter, MethodRouter},
};
use enum_dispatch::enum_dispatch;

async fn stupid() -> String {
    "Hi there".into()
}

#[enum_dispatch(ServiceType<H, T, S, B>)]
trait Service<S = (), B = Body> {
    fn extract_service(&mut self) -> MethodRouter<S, B>;
}

#[enum_dispatch]
pub enum ServiceType<H, T, S, B>
where
    H: Handler<T, S, B>,
    B: HttpBody + Send + 'static,
    T: 'static,
    S: Clone + Send + Sync + 'static,
{
    ByohService(ByohService<H, T, S, B>),
}

pub struct ByohService<H, T, S, B>
where
    H: Handler<T, S, B>,
    B: HttpBody + Send + 'static,
    T: 'static,
    S: Clone + Send + Sync + 'static,
{
    supported_methods: MethodFilter,
    handler: H,
    phantom: PhantomData<(T, S, B)>
}

// pub struct test<H, S, T, B> where
//     H: Handler<T, S, B>,
//     B: HttpBody + Send + 'static,
//     T: 'static,
//     S: Clone + Send + Sync + 'static {
//         mine: H
//         }

impl<H, T, S, B> Service<S, B> for ByohService<H, T, S, B>
where
    H: Handler<T, S, B>,
    B: HttpBody + Send + 'static,
    T: 'static,
    S: Clone + Send + Sync + 'static,
{
    fn extract_service(&mut self) -> MethodRouter<S, B> {
        on(self.supported_methods, self.handler.clone())
    }
}

fn main() {
    let mut serv = ServiceType::ByohService(ByohService {
        supported_methods: MethodFilter::GET,
        handler: stupid,
        phantom: PhantomData
    });
    // serv.extract_service();
    println!("Hello, world!");
}
