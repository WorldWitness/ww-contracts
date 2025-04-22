use anchor_lang::prelude::{ Pubkey,*};


#[account]
pub struct Testimony{
    pub presence_challenge_key : Pubkey,
    pub cid : Vec<u8>,

    //Signed Merkle root of uploaded file
    pub signature : [u8; 32],

    //The actions take by this node for the perscribed PresenceChallenge
    // The encrypted control command pk is a pk encrypted by the Witness Node's public key
    // The control command pk is used to encrypt the control commands
    pub encrypted_control_commands : Vec<Vec<u8>>,
    pub encrypted_control_commands_pk : Vec<u8>
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ControlCommand{

}