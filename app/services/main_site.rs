use framework::services::{ServiceType, StaticDataService};

pub fn get_service() -> ServiceType {
    StaticDataService::default().into()
}
