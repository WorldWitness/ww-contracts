pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("DBARz9f17bZT6infskfhWpepU8aj46nCN7p7hf7pFKxC");

#[program]
pub mod location_epoch {
    use super::*;

    pub fn create_new_location_epoch(ctx: Context<CreateNewLocationEpoch>) -> Result<()> {
        create_new_location_epoch::handler(ctx)
    }

    pub fn submit_witness_testimony(ctx: Context<CreateNewLocationEpoch>) -> Result<()> {
        unimplemented!()
    }

    pub fn transition_location_epoch_state(ctx: Context<CreateNewLocationEpoch>) -> Result<()> {
        unimplemented!()
    }
}
