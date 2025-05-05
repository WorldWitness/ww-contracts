use std::cmp::max_by;

use anchor_lang::prelude::*;
use witness_manager::WitnessNode;
use crate::{error::LocationEpochError, LocationEpoch, LocationEpochPhase, Testimony};
use location_registry::RegisteredLocation;



#[derive(Accounts)]
pub struct SubmitWitnessTestimony<'info> {


    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub witness_node: Account<'info, WitnessNode>,

    #[account(
        init,
        payer = signer,
        seeds = [b"testimony", witness_node.key().as_ref(), location_epoch.key().as_ref()],
        bump,
        space = 8
    )]
    pub testimony : Account<'info, Testimony>,


    #[account(mut)]
    pub location_epoch: Account<'info, LocationEpoch>,

    pub system_program: Program<'info, System>,
}


pub fn handler(ctx: Context<SubmitWitnessTestimony>) -> Result<()> {

    require!(ctx.accounts.witness_node.state.authority == ctx.accounts.signer.key(), LocationEpochError::CustomError);

    // Send default increment phase if it is time
    require!(ctx.accounts.location_epoch.phase == LocationEpochPhase::TestimonySubmission, LocationEpochError::CustomError);

    // Witness node must be enabled, signer must own witness node, witness location must match location for location epoch

    // let curent_challenge_index = ctx.accounts.witness_node.state.num_challenges_requested.checked_sub(1).unwrap();
    // // let witness_node = ctx.accounts.witness_node;
    // let presence_challenge_key = Pubkey::find_program_address(&[b"presence_challenge", &curent_challenge_index.to_be_bytes()[..]],  &location_registry::id()).0;
    // require!(presence_challenge_key == ctx.accounts.presence_challenge.key(), LocationEpochError::CustomError);
    // require
    

    ctx.accounts.location_epoch.num_testimonies += 1;

    Ok(())




}
