use anchor_lang::prelude::*;
use crate::{events, location_registry_error::LocationRegistryErrorCode, state::*, LocationCreatedEvent, LOCATION };

#[derive(Accounts)]
pub struct CreateLocation<'info> {

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub location_counter: Account<'info, LocationCounter>,

    #[account(
        init,
        payer = payer,
        seeds = [LOCATION.as_bytes(), &location_counter.num_locations.to_le_bytes()],
        bump,
        space = 8 + std::mem::size_of::<RegisteredLocation>(),
    )]
    pub location: Account<'info, RegisteredLocation>,

    pub system_program: Program<'info, System>,
}


pub fn handler(
    ctx: Context<CreateLocation>,
    given_metadata :  LocationMetadata,
    given_policy : LocationPolicy

) -> Result<()> {

    let counter = &mut ctx.accounts.location_counter;
    require!(!counter.is_frozen, LocationRegistryErrorCode::LocationCreationFrozen);
    counter.num_locations += 1;


    let new_stats = &mut ctx.accounts.location.stats;
    new_stats.is_live = false;
    new_stats.num_segments = 0;

    let new_metadata = &mut ctx.accounts.location.metadata;
    *new_metadata = given_metadata;

    let new_policy = &mut ctx.accounts.location.policy;
    *new_policy = given_policy;

    emit!(LocationCreatedEvent{ index: counter.num_locations  });

    return Ok(());
}