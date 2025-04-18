use anchor_lang::prelude::{ Pubkey,*};


#[account]
pub struct Testimony{
    pub presence_challenge_key : Pubkey,
    pub cid : Vec<u8>,
    pub signature : [u8; 32]
}

