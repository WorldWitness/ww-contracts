use anchor_lang::prelude::*;

use crate::{state::LocationCounter, constants::LOCATION_COUNTER_SIZE};


#[derive(Accounts)]
pub struct InitializeLocationRegistry<'info> {
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

pub fn handler(ctx: Context<InitializeLocationRegistry>) -> Result<()> {
    let location_counter = &mut ctx.accounts.location_counter;
    location_counter.num_locations = 0;
    location_counter.is_frozen = true;
    Ok(())
}
