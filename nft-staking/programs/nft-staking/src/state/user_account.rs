use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserAccount{
    pub points : u32,
    pub amount_stacked : u8,
    pub bump : u8
}