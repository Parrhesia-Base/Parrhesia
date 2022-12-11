use framework::services::{MethodFilter, ServiceType, StaticDataService};

/// Creates a simple static data service that can be used to ensure the
/// backend is operating successfully
pub fn get_service() -> ServiceType {
    let mut service = StaticDataService::default();
    service.supported_methods = MethodFilter::GET;
    service.into()
}
