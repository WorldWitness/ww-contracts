use anchor_lang::prelude::*;
use location_registry::RegisteredLocation;

use crate::{WitnessNode, witness_error::WitnessErrorCode};

#[derive(Accounts)]
pub struct ToggleWitnessEnableState<'info> {

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub witness_node: Account<'info, WitnessNode>,

    #[account(mut)]
    pub location: Account<'info, RegisteredLocation>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<ToggleWitnessEnableState>) -> Result<()> {

    require!(ctx.accounts.witness_node.state.authority == ctx.accounts.signer.key(), WitnessErrorCode::CallerIsNotAuthority);
    require!(ctx.accounts.location.key() == ctx.accounts.witness_node.location_key, WitnessErrorCode::WitnessLocationDoesNotMatch );


    if !ctx.accounts.witness_node.state.enabled {
        //TODO update this in the future to allow lower stakes for higher reputation Witnesses
        require!(ctx.accounts.witness_node.state.deposit >= ctx.accounts.location.policy.min_witness_stake, WitnessErrorCode::WitnessLocationDoesNotMatch );
    }
    else{
        //Prevent disabling if it has Testimonies being checked. Not sure how to implement rn
        unimplemented!()
    }

    ctx.accounts.witness_node.state.enabled = !ctx.accounts.witness_node.state.enabled;

    Ok(())
}
