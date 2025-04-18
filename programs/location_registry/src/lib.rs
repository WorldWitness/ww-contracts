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

declare_id!("6voAxfkWGYpi8MJSPbrrMQ9yoGumQmyr7fxW1b5zNfdZ");

#[program]
pub mod location_registry_mint {
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

    pub fn create_new_segment(ctx: Context<NewSegmentRequest>) -> Result<()> {
        handle_create_new_segment(ctx)
    }
}
