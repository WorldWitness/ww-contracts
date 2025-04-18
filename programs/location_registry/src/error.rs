use anchor_lang::prelude::*;

#[error_code]
pub enum LocationErrorCodes {
    #[msg("Location Creation is frozen! Unable to increment Location Counter.")]
    LocationCreationFrozen,

    LocationIsNotLive,

    NewSpacetimeSegmentTooEarly
}
