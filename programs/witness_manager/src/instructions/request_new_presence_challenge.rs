use std::cmp::max_by;

use location_registry::{RegisteredLocation, LocationEpoch};
use anchor_lang::prelude::*;

use crate::{PresenceChallenge, WitnessErrorCode, WitnessNode};


#[derive(Accounts)]
pub struct RequestNewPresenceChallenge<'info> {


    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub witness_node: Account<'info, WitnessNode>,

    #[account(
        init,
        payer = signer,
        seeds = [b"presence_challenge", &witness_node.state.num_challenges_requested.to_be_bytes()[..]],
        bump,
        space = 8
    )]
    pub presence_challenge: Account<'info, PresenceChallenge>,

    pub location: Account<'info, RegisteredLocation>,

    pub location_epoch: Account<'info, LocationEpoch>,

    pub system_program: Program<'info, System>,
}


pub fn handler(ctx: Context<RequestNewPresenceChallenge>) -> Result<()> {

    require!(ctx.accounts.witness_node.state.authority == ctx.accounts.signer.key(), WitnessErrorCode::CallerIsNotAuthority);
    require!(ctx.accounts.witness_node.state.enabled, WitnessErrorCode::WitnessIsNotEnabled);

    let location_key = ctx.accounts.location.key();
    require!(ctx.accounts.location.key() == ctx.accounts.witness_node.location_key, WitnessErrorCode::WitnessLocationDoesNotMatch );


    let current_segment_index = ctx.accounts.location.stats.num_segments.checked_sub(1).unwrap();
    let current_spacetime_seg_key = Pubkey::find_program_address(&[location_key.key().as_ref(), &current_segment_index.to_le_bytes() ],  &location_registry::id()).0;
    require!(ctx.accounts.location_epoch.key() == current_spacetime_seg_key, WitnessErrorCode::NotCurrentSpacetimeSegment);
    require!(ctx.accounts.witness_node.state.deposit >= ctx.accounts.location_epoch.issued_policy.min_witness_stake, WitnessErrorCode::NotEnoughFundsinDeposit );

    


    let current_time = Clock::get()?.unix_timestamp;
    require!(current_time > ctx.accounts.witness_node.state.next_challenge_unlockable_at, WitnessErrorCode::TooEarlyToUnlockNextChallenge);

    // // Check

    ctx.accounts.witness_node.state.num_challenges_requested += 1;
    let unlocked_at = current_time + ctx.accounts.location.policy.presence_challenge_duration;
    ctx.accounts.witness_node.state.next_challenge_unlockable_at = std::cmp::max(unlocked_at, ctx.accounts.witness_node.state.next_challenge_unlockable_at);


    ctx.accounts.presence_challenge.begins_at = current_time;
    ctx.accounts.presence_challenge.expires_at = ctx.accounts.witness_node.state.next_challenge_unlockable_at;
    ctx.accounts.presence_challenge.location_epoch_key = ctx.accounts.location_epoch.key();


    // ctx.accounts.sys





    unimplemented!()





}