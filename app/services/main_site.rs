use framework::services::{MethodFilter, ServiceType, StaticDataService};

pub fn get_service() -> ServiceType {
    let mut service = StaticDataService::default();
    service.supported_methods = MethodFilter::GET;
    service.into()
}
