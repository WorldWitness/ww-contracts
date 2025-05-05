use std::cmp::max_by;

use anchor_lang::prelude::*;
use location_epoch_manager::{error::LocationEpochError, LocationEpoch, LocationEpochPhase, Testimony};
use witness_manager::WitnessNode;

use crate::{TestimonyReview, VerifierEpochCard, VerifierNode};



#[derive(Accounts)]
pub struct SubmitTestimonyReview<'info> {


    #[account(mut)]
    pub signer: Signer<'info>,

    pub testimony : Account<'info, Testimony>,

    pub location_epoch: Account<'info, LocationEpoch>,



    #[account(
        init,
        payer = signer,
        seeds = [b"testimony_review", verifier_node.key().as_ref(), testimony.key().as_ref()],
        bump,
        space = 8
    )]
    pub testimony_review : Account<'info, TestimonyReview>,


    #[account(mut)]
    pub verifier_epoch_card: Account<'info, VerifierEpochCard>,

    #[account(mut)]
    pub verifier_node: Account<'info, VerifierNode>,

    pub system_program: Program<'info, System>,
}


pub fn handler(ctx: Context<SubmitTestimonyReview>) -> Result<()> {

    // Make sure testimony, verifier_epoch_card coresspond to same location_epoch
    // Signer needs to own VerifierNOde and VerifierNode owns verifier epoch card
    

    // If a testimony review is created successfully, then increment verifier epoch card


    Ok(())
}
