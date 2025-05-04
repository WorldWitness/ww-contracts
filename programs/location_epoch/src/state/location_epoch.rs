use anchor_lang::prelude::{ Pubkey,*};
use location_registry::LocationPolicy;



#[account]
pub struct LocationEpoch{
    pub start_time : i64,
    pub end_time : i64,
    pub issued_policy : LocationPolicy,
    pub num_testimonies : u64,
    pub phase : EpochPhase
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum EpochPhase{
    TestimonySubmission,
    VerifierRegistration,
    VerifierVote,
    VerifierReveal,
    WitnessStateReveal,
    WitnessVoteAggregation,
    WitnessSelection,
    AggregatorSelection,
    AggregatorSubmission,
    AggregatorVote,
    AggregatorReveal,
}