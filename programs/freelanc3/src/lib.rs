pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("9KRxraNeLy54F2ZR5fVsddBFVZFvQEoETaee1gNypY2G");

#[program]
pub mod freelanc3 {

    use super::*;

    pub fn create_profile(
        ctx : Context<InitProfile>, 
        username : String 
    )-> Result<()>{
        init_profile::create_profile(ctx, username)
    }

    pub fn create_job(
        ctx : Context<CreateJob>,
        job_id : u64, 
    title : String, 
    budget : u32 , 
    )-> Result<()>{
        create_job::create_job(ctx, job_id, title, budget)
    }

    pub fn apply_job(
        ctx : Context<ApplyJob>,
        resume_ref : String
    )-> Result<()>{
        apply_job::apply_job(ctx, resume_ref)
    }

    pub fn hire(
        ctx : Context<Hire>,
    )-> Result<()>{
        hire::hire(ctx)
    }

    
}
