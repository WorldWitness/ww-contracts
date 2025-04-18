use anchor_lang::prelude::{ Pubkey,*};

use super::LocationPolicy;


#[account]
pub struct RegisteredLocation {
    pub stats : LocationStats,
    pub policy : LocationPolicy,
    pub metadata : LocationMetadata
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct LocationStats {
    pub num_segments: u128,
    pub is_live : bool,
    pub last_created :i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct LocationMetadata {
    pub name: String,
    pub description: String,
    pub bounding_box: BoundingBox,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct BoundingBox {
    pub min_lat: f64,
    pub max_lat: f64,
    pub min_lng: f64,
    pub max_lng: f64,
}