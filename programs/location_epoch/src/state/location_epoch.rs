use anchor_lang::prelude::{ Pubkey,*};

use super::LocationPolicy;


#[account]
pub struct LocationEpoch{
    pub start_time : i64,
    pub end_time : i64,
    pub issued_policy : LocationPolicy,
    pub num_testimonies : u64
}