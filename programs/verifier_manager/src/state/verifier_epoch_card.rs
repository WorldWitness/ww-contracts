use anchor_lang::prelude::{ Pubkey,*};





#[account]
pub struct VerifierEpochCard{
    pub authority : Pubkey,
    pub location_epoch_pk : Pubkey,
    pub num_reviews : u64,
}