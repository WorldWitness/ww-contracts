pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("27J4UkeFkTWmkz7dBv3JLM72pPN3DxRocDTigNhBgKqH");

#[program]
pub mod witness {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn change_witness_capabilities(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn change_witness_location(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn enable_witness(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn disable_witness(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn commit_stake(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn recliam_stake(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
    
    pub fn request_new_presence_challenge(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn submit_witness_testimony(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }


}
