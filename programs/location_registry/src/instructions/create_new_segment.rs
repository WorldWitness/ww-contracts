
use anchor_lang::prelude::*;
use crate::{error::{self, LocationErrorCodes}, state::*, POLICY};

#[derive(Accounts)]
pub struct NewSegmentRequest<'info>{
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub location: Account<'info, Location>,

    #[account(
        seeds = [POLICY.as_bytes(), location.key().as_ref()],
        bump
    )]
    pub location_policy: Account<'info, LocationPolicy>,

    #[account(
        init,
        payer = payer,
        space = 8,
        seeds = [location.key().as_ref(), &location.segment_index.to_le_bytes()],
        bump
    )]
    pub new_segment: Account<'info, SpacetimeSegment>,

    pub system_program: Program<'info, System>,
}

pub fn handle_create_new_segment(ctx: Context<NewSegmentRequest>) -> Result<()> {

    require!(ctx.accounts.location.is_live ,error::LocationErrorCodes::LocationIsNotLive);

    let current_time : i64 = Clock::get()?.unix_timestamp;
    require!(ctx.accounts.location_policy.segment_duration + ctx.accounts.location.last_created >  current_time, LocationErrorCodes::NewSpacetimeSegmentTooEarly) ;

    ctx.accounts.location.segment_index += 1;
    ctx.accounts.location.last_created = current_time;
    ctx.accounts.new_segment.start_time = current_time;
    ctx.accounts.new_segment.end_time = current_time + ctx.accounts.location_policy.segment_duration;
    

    Ok(())
}
