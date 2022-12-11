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
    
    pub fn set_handler<H, T>( &mut self, handler: H ) where
        H: Handler<T, (), Body>,
        T: 'static {
        self.subrouter = Some( on( self.supported_methods, handler ) );
    }

    /// Returns an error result if set_handler is called prior to this method
    pub fn set_methods( &mut self, methods: MethodFilter ) -> Result<()> {
        if self.subrouter.is_some() {
            bail!( "Supported Methods must be set prior to setting the handler!" )
        }
        self.supported_methods = methods;
        Ok(())
    }
}

fn main() {
    let mut j = ByohService::new();
    j.set_handler( stupid );
}
