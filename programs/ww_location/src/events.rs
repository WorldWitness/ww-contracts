
use anchor_lang::prelude::*;

#[event]
pub struct LocationCreatedEvent{
    pub index : u128,
}