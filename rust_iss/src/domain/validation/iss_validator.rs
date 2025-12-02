use crate::domain::iss::IssCreateRequest;
use validator::{Validate, ValidationError};

#[derive(Debug, Validate)]
pub struct IssDataValidator {
    #[validate(range(min = -90.0, max = 90.0))]
    pub latitude: f64,
    
    #[validate(range(min = -180.0, max = 180.0))]
    pub longitude: f64,
    
    #[validate(range(min = 0.0))]
    pub altitude: f64,
    
    #[validate(range(min = 0.0))]
    pub velocity: f64,
    
    #[validate(length(min = 1, max = 50))]
    pub visibility: String,
}

impl IssDataValidator {
    pub fn from_request(req: &IssCreateRequest) -> Self {
        Self {
            latitude: req.latitude,
            longitude: req.longitude,
            altitude: req.altitude,
            velocity: req.velocity,
            visibility: req.visibility.clone(),
        }
    }
    
}

