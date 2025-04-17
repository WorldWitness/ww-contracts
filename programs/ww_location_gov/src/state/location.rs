use anchor_lang::prelude::{ Pubkey,*};

#[account]
pub struct LocationCounter {
    pub current_index: u128,
    pub is_frozen : bool
}

#[account]
pub struct Location {
    pub current_segment_pk : Option<Pubkey>,
    pub is_live : bool,
}

#[account]
pub struct SpacetimeSegment{
    pub previous_segment_pk : Option<Pubkey>,
    pub start_time : i64,
    pub end_time : i64
}