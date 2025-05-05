use anchor_lang::prelude::{ Pubkey,*};





#[account]
pub struct VerifierNode{
    pub state : VerifierState,
    
}



#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct VerifierState{
    pub deposit : u64,
    pub enabled : bool,
    pub authority : Pubkey
}
