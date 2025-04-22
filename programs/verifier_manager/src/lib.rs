pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("8GTW8wQccLPTpVGHghTg34nJ7xu8k1cK3gNiq9FzukuA");

#[program]
pub mod verifier_manager {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn toggle_witness_enable_state(ctx:  Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn commit_to_deposit(ctx:  Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn reclaim_from_deposit(ctx:  Context<Initialize>) -> Result<()> {
        Ok(())
    }
}
