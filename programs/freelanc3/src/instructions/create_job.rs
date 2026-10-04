use anchor_lang::prelude::*;
use crate::error::ErrorCode; 


use crate::Job; 

#[derive(Accounts)]
#[instruction(job_id : u64)]
pub struct CreateJob<'info>{

    #[account(mut)]
    pub poster : Signer<'info>, 

    #[account(
        init, 
        payer = poster, 
        space = 8 + Job::INIT_SPACE, 
        seeds = [b"job", poster.key().as_ref() , &job_id.to_le_bytes()], 
        bump
    )]
    pub job : Account<'info, Job>, 

    pub system_program : Program<'info,System> 

}

pub fn create_job(
    ctx : Context<CreateJob>, 
    job_id : u64, 
    title : String, 
    budget : u32 , 

)-> Result<()>{

    require!(
        !title.is_empty() && title.len() <= 32,
        ErrorCode::InvalidTitle
    ); 

    require!(
        budget > 0, 
        ErrorCode::InvalidBudget
    ); 

    let job = &mut ctx.accounts.job;
    job.poster = ctx.accounts.poster.key(); 
    job.job_id = job_id; 
    job.title = title; 
    job.status = crate::Status::Opened; 
    job.budget = budget; 
    job.bump = ctx.bumps.job; 

    Ok(())
}