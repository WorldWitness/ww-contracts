use anchor_lang::prelude::{ Pubkey,*};

#[account]
pub struct LocationMetadata {
    pub name: String,
    pub description: String,
    pub bounding_box: BoundingBox,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct LocationMetadataConfig {
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