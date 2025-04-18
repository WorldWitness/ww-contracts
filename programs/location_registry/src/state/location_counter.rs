use anchor_lang::prelude::{ Pubkey,*};

#[account]

pub struct LocationCounter {
    pub num_locations: u128,
    pub is_frozen : bool
}
