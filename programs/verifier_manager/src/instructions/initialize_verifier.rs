use anchor_lang::prelude::*;

use crate::VerifierNode;

#[derive(Accounts)]
pub struct InitializeVerifier<'info> {

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        seeds = [b"verifier_node", signer.key().as_ref()],
        bump,
        space = 8
    )]
    pub verifier_node: Account<'info, VerifierNode>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeVerifier>) -> Result<()> {
    ctx.accounts.verifier_node.state.deposit = 0;
    ctx.accounts.verifier_node.state.enabled = false;
    ctx.accounts.verifier_node.state.authority = ctx.accounts.signer.key();
    Ok(())
}
