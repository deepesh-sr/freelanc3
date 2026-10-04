use anchor_lang::prelude::*;
use crate::{Application, ApplicationtStatus, Job, Status, error::ErrorCode}; 

#[derive(Accounts)]
pub struct Hire<'info> {

    pub poster : Signer<'info>, 

    #[account(
        mut, 
        has_one = poster @ ErrorCode::NotAPoster, 
        constraint = job.status == Status::Opened @ ErrorCode::InvalidJob
    )]
    pub job : Account<'info, Job>,

    #[account(
        mut, 
        constraint = application.job.key() == job.key() @ ErrorCode::InvalidJob, 
        constraint = application.status == ApplicationtStatus::Applied @ ErrorCode::NotApplied
    )]
    pub application : Account<'info, Application>
}


pub fn hire(
    ctx : Context<Hire>
)-> Result<()>{

    
    let job = &mut ctx.accounts.job; 
    job.status = Status::Hired;
    job.hired_applicant = Some(ctx.accounts.application.applicant); 

    let application = &mut ctx.accounts.application; 
    application.status = ApplicationtStatus::Accepted; 
    Ok(())
}