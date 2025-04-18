use anchor_lang::prelude::{ Pubkey,*};


#[account]
pub struct PresenceChallenge{
    pub begins_at : i64,
    pub expires_at : i64,
    pub spacetime_segment_key : Pubkey,
    pub control_commands : Vec<ControlCommand>
}



#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ControlCommand{

}