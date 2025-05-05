pub mod constants;
pub mod instructions;
pub mod state;
pub mod events;
pub mod location_registry_error;
use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;
pub use events::*;
pub use location_registry_error::*;

declare_id!("6voAxfkWGYpi8MJSPbrrMQ9yoGumQmyr7fxW1b5zNfdZ");

#[program]
pub mod location_registry {
    use super::*;

    pub fn initialize(ctx: Context<InitializeLocationRegistry>) -> Result<()> {
        initialize_location_registry::handler(ctx)
    }

    pub fn create_location(ctx: Context<CreateLocation>, metadata :  LocationMetadata,
        policy : LocationPolicy) -> Result<()> {
        create_location::handler(ctx, metadata, policy)
    }

    pub fn toggle_location_creation(ctx: Context<ToggleLocationCreation>) -> Result<()> {
        toggle_location_creation::handler(ctx)
    }

    pub fn change_location_metadata(ctx: Context<ToggleLocationCreation>, location_index : u128, metadata :  LocationMetadata ) ->Result<()>{
        Ok(())
    }

    pub fn change_location_policy(ctx: Context<ToggleLocationCreation>, location_index : u128, policy :  LocationPolicy ) ->Result<()>{
        Ok(())
    }

    // pub fn create_new_segment(ctx: Context<CreateNewLocationEpoch>) -> Result<()> {
    //     create_new_location_epoch::handler(ctx)
    // }
}
