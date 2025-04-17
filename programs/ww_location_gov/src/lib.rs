pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod events;
use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;
pub use events::*;

declare_id!("8oer2N17ZmMGyE3xJepNjqi2hn1EU4CwUeSWwB38PPdZ");

#[program]
pub mod ww_location_gov {
    use super::*;

    pub fn initialize(ctx: Context<InitializeLocation>) -> Result<()> {
        handle_initialize_location_gov(ctx)
    }

    pub fn create_location(ctx: Context<CreateLocation>, metadata :  LocationMetadataConfig,
        policy : LocationPolicyConfig) -> Result<()> {
        handle_create_location(ctx, metadata, policy)
    }

    pub fn disable_location_creation(ctx: Context<DisableLocationCreation>) -> Result<()> {
        handle_disable_location_creation(ctx)
    }

    pub fn enable_location_creation(ctx: Context<EnableLocationCreation>) -> Result<()> {
        handle_enable_location_creation(ctx)
    }

    pub fn change_location_metadata(ctx: Context<EnableLocationCreation>, location_index : u128, metadata :  LocationMetadataConfig ) ->Result<()>{
        Ok(())
    }

    pub fn change_location_policy(ctx: Context<EnableLocationCreation>, location_index : u128, policy :  LocationPolicyConfig ) ->Result<()>{
        Ok(())
    }
}
