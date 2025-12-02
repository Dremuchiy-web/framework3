use crate::domain::astronomy::AstronomyCreateRequest;
use validator::Validate;

#[derive(Debug, Validate)]
pub struct AstronomyDataValidator {
    #[validate(length(min = 1, max = 255))]
    pub object_name: String,
    
    #[validate(length(min = 1, max = 100))]
    pub object_type: String,
    
    #[validate(range(min = 0.0, max = 360.0))]
    pub ra: f64,
    
    #[validate(range(min = -90.0, max = 90.0))]
    pub dec: f64,
}

impl AstronomyDataValidator {
    pub fn from_request(req: &AstronomyCreateRequest) -> Self {
        Self {
            object_name: req.object_name.clone(),
            object_type: req.object_type.clone(),
            ra: req.ra,
            dec: req.dec,
        }
    }
}

