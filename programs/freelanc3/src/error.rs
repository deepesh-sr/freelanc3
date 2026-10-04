use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,
    #[msg("Invalid username")]
    InvalidUsername,
    #[msg("Invalid Title")]
    InvalidTitle,
    #[msg("Budget less than 0")]
    InvalidBudget,
    #[msg("Job is not open")]
    InvalidJob,
    #[msg("Invalid Resume Reference")]
    InvalidResumeRef,
    #[msg("Not a poster of the job ")]
    NotAPoster,
    #[msg("Has not applied for the job")]
    NotApplied,
    
    
}
