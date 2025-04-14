
use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct EnableLocationCreation<'info> {

    #[account(mut)]
    pub location_counter: Account<'info, LocationCounter>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}


pub fn handle_enable_location_creation(ctx: Context<EnableLocationCreation>) -> Result<()> {
    let location_counter = &mut ctx.accounts.location_counter;
    location_counter.is_frozen = false;
    Ok(())
}