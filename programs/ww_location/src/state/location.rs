use anchor_lang::prelude::{ Pubkey,*};

#[account]
pub struct LocationCounter {
    pub current_index: u128,
    pub is_frozen : bool
}

#[account]
pub struct Location {
    pub segment_index: u128,
    pub is_live : bool,
    pub last_created :i64
}

#[account]
pub struct SpacetimeSegment{
    pub start_time : i64,
    pub end_time : i64
}