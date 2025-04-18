use std::cmp::max_by;

use location_registry::{RegisteredLocation, SpacetimeSegment};
use anchor_lang::prelude::*;

use witness_manager::{witness, PresenceChallenge, WitnessNode};

#[derive(Accounts)]
pub struct RequestNewPresenceChallenge<'info> {


    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub witness_node: Account<'info, WitnessNode>,

    #[account(
        init,
        payer = signer,
        seeds = [b"presence_challenge", signer.key().as_ref(), &witness_node.state.num_challenges_requested.to_le_bytes()],
        bump,
        space = 8
    )]
    pub presence_challenge: Account<'info, PresenceChallenge>,


    pub spacetime_segment: Account<'info, SpacetimeSegment>,

    pub system_program: Program<'info, System>,
}


pub fn handler(ctx: Context<RequestNewPresenceChallenge>) -> Result<()> {

    // require!(ctx.accounts.witness_node.state.authority == ctx.accounts.signer.key(), WitnessErrorCode::CallerIsNotAuthority);
    // require!(ctx.accounts.witness_node.state.enabled, WitnessErrorCode::WitnessIsNotEnabled);
    // require!(ctx.accounts.location.key() == ctx.accounts.witness_node.location_key, WitnessErrorCode::WitnessLocationDoesNotMatch );
    // require!(ctx.accounts.witness_node.state.deposit >= ctx.accounts.spacetime_segment.issued_policy.min_witness_stake, WitnessErrorCode::NotEnoughFundsinDeposit );

    


    // let current_time = Clock::get()?.unix_timestamp;
    // require!(current_time > ctx.accounts.witness_node.state.next_challenge_unlockable_at, WitnessErrorCode::TooEarlyToUnlockNextChallenge);

    // // Check

    // ctx.accounts.witness_node.state.num_challenges_requested += 1;
    // let unlocked_at = current_time + ctx.accounts.location.policy.presence_challenge_duration;
    // ctx.accounts.witness_node.state.next_challenge_unlockable_at = std::cmp::max(unlocked_at, ctx.accounts.witness_node.state.next_challenge_unlockable_at);


    // ctx.accounts.presence_challenge.begins_at = current_time;
    // ctx.accounts.presence_challenge.expires_at = ctx.accounts.witness_node.state.next_challenge_unlockable_at;
    // ctx.accounts.presence_challenge.spacetime_segment_key = ctx.accounts.spacetime_segment.key();

    // ctx.accounts.sys





    unimplemented!()





}