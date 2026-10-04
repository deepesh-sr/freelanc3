use anchor_lang::prelude::*;
use crate::{Application, Job, Status, error::ErrorCode}; 


#[derive(Accounts)]
pub struct ApplyJob<'info> {

    #[account(mut)]
    pub applicant : Signer<'info>, 

    pub job : Account<'info, Job>, 

    #[account(
        init, 
        payer = applicant,
        space = 8 + Application::INIT_SPACE, 
        seeds = [b"application" , job.key().as_ref() , applicant.key().as_ref()],
        bump
    )]
    pub application : Account<'info, Application>, 

    pub system_program : Program<'info, System> 
}


pub fn apply_job(
    ctx : Context<ApplyJob>, 
    resume_ref : String 
)-> Result<()>{

    require!(
        ctx.accounts.job.status == Status::Opened, 
        ErrorCode::InvalidJob
    );

    require!(
        !resume_ref.is_empty() && resume_ref.len() <= 64, 
        ErrorCode::InvalidResumeRef
    ); 

    let application = &mut ctx.accounts.application; 

    application.applicant = ctx.accounts.applicant.key(); 
    application.job = ctx.accounts.job.key(); 
    application.resume_ref = resume_ref; 
    application.status = crate::ApplicationtStatus::Applied; 
    application.bump = ctx.bumps.application; 

    Ok(())
}