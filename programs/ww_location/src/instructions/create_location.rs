use anchor_lang::prelude::*;
use crate::{error::{LocationErrorCodes}, events, state::*, LocationCreatedEvent, LOCATION_COUNTER};

#[derive(Accounts)]
pub struct CreateLocation<'info> {

    #[account(mut)]
    pub location_counter: Account<'info, LocationCounter>,

    #[account(
        init,
        payer = payer,
        seeds = [LOCATION_COUNTER.as_bytes(), &location_counter.current_index.to_le_bytes()],
        bump,
        space = 8 + std::mem::size_of::<LocationAccount>(),
    )]
    pub location: Account<'info, LocationAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}


pub fn handle_create_location(
    ctx: Context<CreateLocation>,
    description: String,
    bounding_box: BoundingBox,
) -> Result<()> {
    let new_location = &mut ctx.accounts.location;
    let counter = &mut ctx.accounts.location_counter;

    require!(!counter.is_frozen, LocationErrorCodes::LocationCreationFrozen);

    new_location.index = counter.current_index;
    new_location.description = description;
    new_location.bounding_box = bounding_box;
    new_location.created_at = Clock::get()?.unix_timestamp;

    counter.current_index += 1;

    emit!(LocationCreatedEvent{ index: new_location.index });

    return Ok(());
}