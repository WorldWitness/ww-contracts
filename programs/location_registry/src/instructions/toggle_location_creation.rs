use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct ToggleLocationCreation<'info> {

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub location_counter: Account<'info, LocationCounter>,

    pub system_program: Program<'info, System>,
}


pub fn handler(ctx: Context<ToggleLocationCreation>) -> Result<()> {
    let location_counter = &mut ctx.accounts.location_counter;
    location_counter.is_frozen = !location_counter.is_frozen;
    Ok(())
}