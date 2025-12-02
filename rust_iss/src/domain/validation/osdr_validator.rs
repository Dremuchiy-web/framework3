use crate::domain::osdr::OsdrCreateRequest;
use validator::Validate;

#[derive(Debug, Validate)]
pub struct OsdrDataValidator {
    #[validate(length(min = 1, max = 255))]
    pub dataset_id: String,
    
    #[validate(length(min = 1, max = 500))]
    pub title: String,
    
    #[validate(range(min = 0))]
    pub file_count: i32,
    
    #[validate(range(min = 0))]
    pub size_bytes: i64,
}

impl OsdrDataValidator {
    pub fn from_request(req: &OsdrCreateRequest) -> Self {
        Self {
            dataset_id: req.dataset_id.clone(),
            title: req.title.clone(),
            file_count: req.file_count,
            size_bytes: req.size_bytes,
        }
    }
}

