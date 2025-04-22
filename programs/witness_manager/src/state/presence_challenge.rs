use anchor_lang::prelude::{ Pubkey,*};


#[account]
pub struct PresenceChallenge{
    pub begins_at : i64,
    pub expires_at : i64,
    pub location_epoch_key : Pubkey,
}



