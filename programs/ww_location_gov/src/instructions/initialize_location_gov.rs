use anchor_lang::prelude::*;

use crate::{state::LocationCounter, LOCATION_COUNTER_SIZE};

#[derive(Accounts)]
pub struct InitializeLocation<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        seeds = [b"location_counter"],
        bump,
        space = 8 + LOCATION_COUNTER_SIZE
    )]
    pub location_counter: Account<'info, LocationCounter>,

    pub system_program: Program<'info, System>,
}

pub fn handle_initialize_location_gov(ctx: Context<InitializeLocation>) -> Result<()> {
    let location_counter = &mut ctx.accounts.location_counter;
    location_counter.current_index = 0;
    location_counter.is_frozen = true;
    Ok(())
}
