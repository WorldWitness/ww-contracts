use anchor_lang::prelude::*;


#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct LocationPolicy{
    pub min_witnesses: u32,
    pub min_verifiers: u32,
    pub min_storage_duration: u32,
    pub segment_duration: i64,
    pub presence_challenge_duration: i64,
    pub testimony_submission_duration: i64,
    pub min_witness_stake : u64,
    pub min_verifier_stake : u64
}