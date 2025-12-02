use crate::domain::jwst::JwstCreateRequest;
use validator::Validate;

#[derive(Debug, Validate)]
pub struct JwstDataValidator {
    #[validate(length(min = 1, max = 255))]
    pub observation_id: String,
    
    #[validate(length(min = 1, max = 255))]
    pub target_name: String,
    
    #[validate(length(min = 1, max = 100))]
    pub instrument: String,
    
    #[validate(length(min = 1, max = 100))]
    pub observation_type: String,
}

impl JwstDataValidator {
    pub fn from_request(req: &JwstCreateRequest) -> Self {
        Self {
            observation_id: req.observation_id.clone(),
            target_name: req.target_name.clone(),
            instrument: req.instrument.clone(),
            observation_type: req.observation_type.clone(),
        }
    }
}

