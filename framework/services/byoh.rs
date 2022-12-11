use super::Service;

use anyhow::{bail, Result};
use axum::{
    body::Body,
    handler::Handler,
    routing::{on, MethodFilter, MethodRouter},
};

/// Bring-Your-Own-Handler Service
pub struct ByohService {
    pub supported_methods: MethodFilter,
    subrouter: Option<MethodRouter>,
}

impl Service for ByohService {
    fn extract_service(&mut self) -> Result<MethodRouter> {
        match std::mem::take(&mut self.subrouter) {
            Some(subrouter) => Ok(subrouter),
            None => bail!("Attempted to use the BYOH service without first giving it a handler!"),
        }
    }
}

impl ByohService {
    pub fn new() -> Self {
        ByohService {
            supported_methods: MethodFilter::empty(),
            subrouter: None,
        }
    }

    pub fn set_handler<H, T>(&mut self, handler: H)
    where
        H: Handler<T, (), Body>,
        T: 'static,
    {
        self.subrouter = Some(on(self.supported_methods, handler));
    }

    /// Returns an error result if set_handler is called prior to this method
    pub fn set_methods(&mut self, methods: MethodFilter) -> Result<()> {
        if self.subrouter.is_some() {
            bail!("Supported Methods must be set prior to setting the handler!")
        }
        self.supported_methods = methods;
        Ok(())
    }
}
