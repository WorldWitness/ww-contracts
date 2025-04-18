use anchor_lang::prelude::{ Pubkey,*};

#[account]
pub struct LocationPolicy {
    pub min_witnesses: u32,
    pub min_verifiers: u32,
    pub segment_duration: i64,
    pub minimum_presence_challenge_time: i64,
    pub testimony_submission_time: i64,
    pub witness_stake : u32
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct LocationPolicyConfig{
    pub min_witnesses: u32,
    pub min_verifiers: u32,
    pub segment_duration: i64,
    pub minimum_presence_challenge_time: i64,
    pub testimony_submission_time: i64,
    pub witness_stake : u32
}