use anchor_lang::prelude::{ Pubkey,*};

use super::LocationPolicy;


#[account]
pub struct SpacetimeSegment{
    pub start_time : i64,
    pub end_time : i64,
    pub issued_policy : LocationPolicy
}