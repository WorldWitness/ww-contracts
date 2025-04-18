use location_registry::RegisteredLocation;
use anchor_lang::prelude::*;

use crate::{WitnessCapabilities, WitnessNode};

#[derive(Accounts)]
#[instruction(seed: String)] 
pub struct InitializeWitness<'info> {


    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        seeds = [b"witness_node", signer.key().as_ref(), seed.as_bytes()],
        bump,
        space = 8
    )]
    pub witness_node: Account<'info, WitnessNode>,

    pub location: Account<'info, RegisteredLocation>,

    pub system_program: Program<'info, System>,


}

pub fn handler(ctx: Context<InitializeWitness>,seed: String, wc : WitnessCapabilities) -> Result<()> {

    ctx.accounts.witness_node.capabilities = wc;
    ctx.accounts.witness_node.state.deposit = 0;
    ctx.accounts.witness_node.state.enabled = false;
    ctx.accounts.witness_node.state.num_challenges_requested = 0;
    ctx.accounts.witness_node.state.authority = ctx.accounts.signer.key();
    ctx.accounts.witness_node.location_key = ctx.accounts.location.key();
    Ok(())
}
