use rocket::http::Status; // https://api.rocket.rs/v0.4/rocket/http/struct.Status.html#structfield.reason


#[derive(Debug)]
pub enum CHEAPError {
    Unknown,
    InvalidStage,
    InvalidUser,
    InvalidSigned,
}

impl CHEAPError {
    pub fn to_status(&self) -> Status {
        match self {
            CHEAPError::Unknown => Status::InternalServerError,
            CHEAPError::InvalidStage => Status::BadRequest,
            CHEAPError::InvalidUser => Status::SeeOther,
            CHEAPError::InvalidSigned => Status::Unauthorized,
        }
    }
}
