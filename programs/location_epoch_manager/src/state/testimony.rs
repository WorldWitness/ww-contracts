use anchor_lang::prelude::{ Pubkey,*};


#[account]
pub struct Testimony{
    pub footage_cid : Vec<u8>,
    pub scene_cid : Vec<u8>,


    pub location_epoch_pk : Pubkey,

    //Signed Merkle root of uploaded file
    pub signature : [u8; 32],

    // The encrypted camera states that Witness Node was in
    pub encrypted_camera_states : Vec<Vec<u8>>,
    pub encrypted_pk : Vec<u8>
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CameraState{

}