use anchor_lang::{prelude::*, solana_program::{program::invoke, system_instruction}};

use crate::{WitnessNode, witness_error::WitnessErrorCode};

#[derive(Accounts)]
pub struct ReclaimFromDeposit<'info> {

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub witness_node: Account<'info, WitnessNode>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<ReclaimFromDeposit>, money_to_reclaim : u64 ) -> Result<()> {

    require!(ctx.accounts.witness_node.state.authority == ctx.accounts.signer.key(), WitnessErrorCode::CallerIsNotAuthority);
    require!(!ctx.accounts.witness_node.state.enabled, WitnessErrorCode::WitnessIsNotDisabled);
    require!(ctx.accounts.witness_node.state.deposit>= money_to_reclaim, WitnessErrorCode::WitnessCanNotReclaimMoreThanDeposit);

    let signer = &ctx.accounts.signer;
    let witness_node = &mut ctx.accounts.witness_node;

    invoke(
        &system_instruction::transfer(
            witness_node.to_account_info().key,
            signer.key,
            money_to_reclaim,
        ),
        &[
            witness_node.to_account_info(),
            signer.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;


    ctx.accounts.witness_node.state.deposit -= money_to_reclaim;

    Ok(())
}