use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow{
    pub seed: u64, //Uniqueness or indexing
    pub maker: Pubkey, 
    pub mint_a: Pubkey, 
    pub mint_b: Pubkey, 
    pub bump : u8,
    pub receive : u64 
}