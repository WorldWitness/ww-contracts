
use anchor_lang::prelude::{ Pubkey,*};


#[account]
pub struct WitnessNode{
    pub capabilities : WitnessCapabilities,
    pub state : WitnessState,
    pub location_key: Pubkey,
    
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct WitnessCapabilities{

}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct WitnessState{
    pub num_challenges_requested : u64,
    pub next_challenge_unlockable_at : i64,
    pub deposit : u64,
    pub enabled : bool,
    pub authority : Pubkey
}

