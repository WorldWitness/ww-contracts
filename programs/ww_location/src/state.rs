use anchor_lang::prelude::*;

#[account]
pub struct LocationCounter {
    pub current_index: u128,
    pub is_frozen : bool
}

#[account]
pub struct LocationAccount {
    pub index: u128,
    pub description: String,
    pub bounding_box: BoundingBox,
    pub created_at: i64,
    pub created_by: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct BoundingBox {
    pub min_lat: f64,
    pub max_lat: f64,
    pub min_lng: f64,
    pub max_lng: f64,
}