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
    #[max_len(32)]
    pub job_id : String,
    #[max_len(32)]
    pub title : String, 
    pub budget : u32,
    pub status : Status
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
    pub status : ApplicationtStatus
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq,Eq, InitSpace)]
pub enum ApplicationtStatus {
Accepted, 
Rejected
}
