use std::convert::Infallible;
use std::error::Error;
use std::marker::PhantomData;

use axum::body::HttpBody;
use axum::{
    body::Body,
    handler::Handler,
    routing::{on, MethodFilter, MethodRouter},
};
use enum_dispatch::enum_dispatch;

use anyhow::{Result, bail};

async fn stupid() -> String {
    "Hi there".into()
}

#[enum_dispatch(ServiceType)]
trait Service {
    fn extract_service(&mut self) -> Result<MethodRouter>;
}

#[enum_dispatch]
pub enum ServiceType
{
    ByohService,
}

pub struct ByohService
{
    supported_methods: MethodFilter,
    subrouter: Option<MethodRouter>,
}

// pub struct test<H, S, T, B> where
//     H: Handler<T, S, B>,
//     B: HttpBody + Send + 'static,
//     T: 'static,
//     S: Clone + Send + Sync + 'static {
//         mine: H
//         }

impl Service for ByohService
{
    fn extract_service(&mut self) -> Result<MethodRouter>  {
        match std::mem::take( &mut self.subrouter ) {
            Some( subrouter ) => Ok( subrouter ),
            None => bail!( "Attempted to use byoh service without proper initialization!" ),
        }
    }
}

impl ByohService
{
    fn new() -> Self {
        ByohService { supported_methods: MethodFilter::empty(), subrouter: None }
    }
    
    fn set_handler<H, T>( &mut self, handler: H ) where
        H: Handler<T, (), Body>,
        T: 'static {
        self.subrouter = Some( on( self.supported_methods, handler ) );
    }
}

fn main() {
    // let mut serv = ServiceType::ByohService(ByohService {
    //     supported_methods: MethodFilter::GET,
    //     handler: stupid,
    //     phantom: PhantomData
    // });
    
    
}
