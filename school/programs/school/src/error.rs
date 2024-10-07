use anchor_lang::prelude::*;

#[error_code]
pub enum NFTError {
    #[msg("Attribute cannot be empty")]
    EmptyAttribute,
    #[msg("The name attribute is too short")]
    InvalidNameLength,
    #[msg("The symbol attribute is too short")]
    InvalidSymbolLength,
    #[msg("Invalid URI")]
    InvalidURI,
    #[msg("Rate limit exceeded")]
    RateLimitExceeded,
}

#[error_code]
pub enum SchoolError {
    #[msg("Invalid school type")]
    InvalidSchoolType,
}

#[error_code]
pub enum CourseError {
    #[msg("Name is too short")]
    InvalidName,
}
