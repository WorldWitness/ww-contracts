use anchor_lang::prelude::*;
use crate::{error::LocationErrorCodes, events, state::*, LocationCreatedEvent, LOCATION, METADATA, POLICY};

#[derive(Accounts)]
pub struct CreateLocation<'info> {

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub location_counter: Account<'info, LocationCounter>,

    #[account(
        init,
        payer = payer,
        seeds = [LOCATION.as_bytes(), &location_counter.current_index.to_le_bytes()],
        bump,
        space = 8 + std::mem::size_of::<Location>(),
    )]
    pub location: Account<'info, Location>,

    #[account(
        init,
        payer = payer,
        seeds = [METADATA.as_bytes(), &location_counter.current_index.to_le_bytes()],
        bump,
        space = 8 + std::mem::size_of::<LocationMetadata>(),
    )]
    pub location_metadata: Account<'info, LocationMetadata>,

    #[account(
        init,
        payer = payer,
        seeds = [POLICY.as_bytes(), &location_counter.current_index.to_le_bytes()],
        bump,
        space = 8 + std::mem::size_of::<LocationPolicy>(),
    )]
    pub location_policy: Account<'info, LocationPolicy>,

    pub system_program: Program<'info, System>,
}


pub fn handle_create_location(
    ctx: Context<CreateLocation>,
    metadata :  LocationMetadataConfig,
    policy : LocationPolicyConfig

) -> Result<()> {

    let counter = &mut ctx.accounts.location_counter;
    require!(!counter.is_frozen, LocationErrorCodes::LocationCreationFrozen);
    counter.current_index += 1;


    let new_location = &mut ctx.accounts.location;

    let new_metadata = &mut ctx.accounts.location_metadata;
    new_metadata.bounding_box = metadata.bounding_box;
    new_metadata.name = metadata.name;
    new_metadata.description = metadata.description;

    let new_policy = &mut ctx.accounts.location_policy;
    new_policy.min_verifiers = policy.min_verifiers;
    new_policy.min_witnesses = policy.min_witnesses;
    new_policy.segment_duration = policy.segment_duration;

    emit!(LocationCreatedEvent{ index: counter.current_index  });

    return Ok(());
}