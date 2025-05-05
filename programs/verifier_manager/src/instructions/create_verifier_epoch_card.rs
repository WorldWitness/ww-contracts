use anchor_lang::prelude::*;
use location_epoch_manager::{LocationEpoch, LocationEpochPhase};
use crate::{error::ErrorCode, state::*};

use crate::{verifier_epoch_card, VerifierEpochCard, VerifierNode};

#[derive(Accounts)]
pub struct CreateVerifierEpochCard<'info> {

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        seeds = [b"verifier_node", verifier_node.key().as_ref(), location_epoch.key().as_ref()],
        bump,
        space = 8
    )]
    pub verifier_epoch_card: Account<'info, VerifierEpochCard>,


    pub verifier_node: Account<'info, VerifierNode>,

    pub location_epoch: Account<'info, LocationEpoch>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateVerifierEpochCard>) -> Result<()> {

    // Make sure signer matches verifier

    require!(ctx.accounts.verifier_node.state.authority.key() == ctx.accounts.signer.key(), ErrorCode::CustomError);


    require!(ctx.accounts.location_epoch.phase == LocationEpochPhase::VerifierRegistration, ErrorCode::CustomError);


    ctx.accounts.verifier_epoch_card.authority = ctx.accounts.verifier_node.key();
    ctx.accounts.verifier_epoch_card.location_epoch_pk = ctx.accounts.location_epoch.key();
    ctx.accounts.verifier_epoch_card.num_reviews = 0;


    Ok(())
}