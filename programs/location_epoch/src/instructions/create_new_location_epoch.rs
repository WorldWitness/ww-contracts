
use anchor_lang::prelude::*;
use location_registry::{RegisteredLocation};
use crate::{error::LocationEpochError, state::*};

#[derive(Accounts)]
pub struct CreateNewLocationEpoch<'info>{
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub location: Account<'info, RegisteredLocation>,

    #[account(
        init,
        payer = payer,
        space = 8,
        seeds = [location.key().as_ref(), &location.stats.num_segments.to_le_bytes()],
        bump
    )]
    pub new_epoch: Account<'info, LocationEpoch>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateNewLocationEpoch>) -> Result<()> {

    require!(ctx.accounts.location.stats.is_live ,LocationEpochError::CustomError);

    let current_time : i64 = Clock::get()?.unix_timestamp;
    require!(ctx.accounts.location.policy.segment_duration + ctx.accounts.location.stats.last_created >  current_time, LocationEpochError::CustomError) ;

    ctx.accounts.location.stats.num_segments += 1;
    ctx.accounts.location.stats.last_created = current_time;
    // ctx.accounts.new_segment.start_time = current_time;
    // ctx.accounts.new_segment.end_time = current_time + ctx.accounts.location.policy.segment_duration;
    // ctx.accounts.new_segment.issued_policy = ctx.accounts.location.policy.clone();
    

    Ok(())
}
