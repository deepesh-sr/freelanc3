use anchor_lang::prelude::*; 

#[account]
#[derive(InitSpace)]
pub struct Profile{
    pub authority : Pubkey, 
    #[max_len(32)]
    pub username : String, 
    pub bump : u8 
}

#[account]
#[derive(InitSpace)]
pub struct Job{
    pub poster : Pubkey, 
    pub job_id : u64,
    #[max_len(32)]
    pub title : String, 
    pub budget : u32,
    pub status : Status, 
    pub bump : u8 
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq,Eq, InitSpace)]
pub enum Status {
    Opened,
    Hired, 
    Completed, 
    Cancelled
}

#[account]
#[derive(InitSpace)]
pub struct Application{
    pub authrity : Pubkey, 
    pub job : Pubkey,
    #[max_len(64)]
    pub resume_ref : String,
    pub status : ApplicationtStatus, 
    pub bump : u8 
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq,Eq, InitSpace)]
pub enum ApplicationtStatus {
Applied, 
Accepted, 
Rejected
}
