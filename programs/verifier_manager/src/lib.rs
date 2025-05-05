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

    pub fn initialize(ctx: Context<InitializeVerifier>) -> Result<()> {
        initialize_verifier::handler(ctx)
    }

    pub fn toggle_verifier_enable_state(ctx:  Context<InitializeVerifier>) -> Result<()> {
        Ok(())
    }

    pub fn commit_to_deposit(ctx:  Context<InitializeVerifier>) -> Result<()> {
        Ok(())
    }

    pub fn reclaim_from_deposit(ctx:  Context<InitializeVerifier>) -> Result<()> {
        Ok(())
    }

    pub fn create_verifier_epoch_card(ctx:  Context<CreateVerifierEpochCard>) ->  Result<()> {
        create_verifier_epoch_card::handler(ctx)
    }

    pub fn submit_tesimony_review(ctx:  Context<CreateVerifierEpochCard>) ->  Result<()> {
        create_verifier_epoch_card::handler(ctx)
    }
}
