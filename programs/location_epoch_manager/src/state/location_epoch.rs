use anchor_lang::prelude::{ Pubkey,*};
use location_registry::LocationPolicy;



#[account]
pub struct LocationEpoch{
    pub current_phase_start_time : i64,
    pub issued_policy : LocationPolicy,
    pub num_testimonies : u64,
    pub phase : LocationEpochPhase
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
#[repr(u8)]
pub enum LocationEpochPhase{
    TestimonySubmission,
    VerifierRegistration,
    VerifierTestimonyReview,
    VerifierReveal,
    WitnessStateReveal,
    WitnessVoteAggregation,
    WitnessSelection,
    AggregatorSelection,
    AggregatorSubmission,
    AggregatorVote,
    AggregatorReveal,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub struct LocationEpochPhaseDuration{
    pub testimony_submission : i64,
    pub verifier_registration : i64,
    pub verifier_testimony_review : i64,
    pub verifier_reveal : i64,
    pub witness_state_reveal : i64,
    pub witness_vote_aggregation : i64,
    pub witness_selection : i64,
    pub aggregator_selection : i64,
    pub aggregator_submission : i64,
    pub aggregator_vote : i64,
    pub aggregator_reveal : i64,
}