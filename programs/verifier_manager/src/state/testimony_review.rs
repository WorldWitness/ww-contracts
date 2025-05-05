use anchor_lang::prelude::{ Pubkey,*};


#[account]
pub struct TestimonyReview{

    pub decision : TestimonyQuality,


    

}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum TestimonyQuality{
    Acceptable{
        // The encrypted camera states that Witness Node was in
        encrypted_camera_states : Vec<Vec<u8>>,
        encrypted_pk : Vec<u8>
    },
    Low,
    Fradulent
}
