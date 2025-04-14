use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

use instructions::{make::* , take::* , close::*};
declare_id!("3TTXmQ7jc1V6fLCaCmv65AkeM4Vmo9TLYGUi6YHj6F1c");

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, receive: u64) -> Result<()> {
        ctx.accounts.initialize(seed, receive, &ctx.bumps)?;
        ctx.accounts.deposit(receive)
    }

    pub fn take(ctx: Context<Take>,amt:u64) -> Result<()> {
        ctx.accounts.take(amt)
    }

    pub fn clsoe(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.close()
    }
}
