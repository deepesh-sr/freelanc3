use anchor_lang::prelude::*;
use crate::error::ErrorCode; 

use crate::Profile;


#[derive(Accounts)]
#[instruction(username : String)]
pub struct InitProfile<'info>{

#[account(mut)]    
pub authority : Signer<'info>, 

#[account(
    init, 
    payer = authority, 
    space = 8 + Profile::INIT_SPACE, 
    seeds = [b"profile", authority.key().as_ref() ],
    bump
)]
pub profile : Account<'info, Profile>,
pub system_program : Program<'info, System> 
}

pub fn create_profile(
    ctx : Context<InitProfile>, 
    username : String
)-> Result<()>{

    require!(
        !username.is_empty() && username.len() <= 32,
        ErrorCode::InvalidUsername
    ); 

    let profile = &mut ctx.accounts.profile; 

    profile.authority = ctx.accounts.authority.key(); 
    profile.username = username; 
    profile.bump = ctx.bumps.profile; 

    Ok(())
}