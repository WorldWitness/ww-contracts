pub mod constants;
pub mod witness_error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;
pub use witness_error::*;

declare_id!("27J4UkeFkTWmkz7dBv3JLM72pPN3DxRocDTigNhBgKqH");

#[program]
pub mod witness_manager {
    use super::*;

    pub fn initialize_witness(ctx: Context<InitializeWitness>, seed: String, wc : WitnessCapabilities) -> Result<()> {
        initialize_witness::handler(ctx, seed, wc)
    }

    pub fn change_witness_capabilities(ctx: Context<InitializeWitness>) -> Result<()> {
        Ok(())
    }

    pub fn change_witness_location(ctx: Context<InitializeWitness>) -> Result<()> {
        Ok(())
    }

    pub fn toggle_witness_enable_state(ctx: Context<ToggleWitnessEnableState>) -> Result<()> {
        toggle_witness_enable_state::handler(ctx)
    }

    pub fn commit_to_deposit(ctx: Context<InitializeWitness>) -> Result<()> {
        Ok(())
    }

    pub fn reclaim_from_deposit(ctx: Context<InitializeWitness>) -> Result<()> {
        Ok(())
    }

    pub fn get_status_of_current_presence_challenge(ctx: Context<InitializeWitness>) -> Result<()> {
        Ok(())
    }
    
    pub fn submit_witness_testimony(ctx: Context<InitializeWitness>) -> Result<()> {
        Ok(())
    }


}
