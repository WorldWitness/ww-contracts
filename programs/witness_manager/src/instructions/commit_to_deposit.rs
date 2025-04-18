use anchor_lang::{prelude::*, solana_program::{program::invoke, system_instruction}};

use crate::{WitnessNode, witness_error::WitnessErrorCode};

#[derive(Accounts)]
pub struct CommitToDeposit<'info> {

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub witness_node: Account<'info, WitnessNode>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CommitToDeposit>, money_to_deposit : u64 ) -> Result<()> {

    require!(ctx.accounts.witness_node.state.authority == ctx.accounts.signer.key(), WitnessErrorCode::CallerIsNotAuthority);

    let signer = &ctx.accounts.signer;
    let witness_node = &mut ctx.accounts.witness_node;

    invoke(
        &system_instruction::transfer(
            signer.key,
            witness_node.to_account_info().key,
            money_to_deposit,
        ),
        &[
            signer.to_account_info(),
            witness_node.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;


    ctx.accounts.witness_node.state.deposit = ctx.accounts.witness_node.state.deposit.checked_add(money_to_deposit).unwrap();

    Ok(())
}
