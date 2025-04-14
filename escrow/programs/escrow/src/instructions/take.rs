use anchor_lang::prelude::*;
use crate::state::Escrow;
use anchor_spl::{
    associated_token::AssociatedToken, token, token_interface::{
        transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked
    }};

#[derive(Accounts)]
pub struct Take<'info>{
    #[account(mut)]
    pub taker : Signer<'info>,
    #[account(
        seeds=[b"escrow",maker.key().as_ref(),escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump
    )]
    pub escrow : Account<'info,Escrow>,

    pub maker: UncheckedAccount<'info>,

    #[account(
        mint::token_program=token_program
    )]
    pub mint_a:InterfaceAccount<'info,Mint>,

    #[account(
        mint::token_program=token_program
    )]
    pub mint_b:InterfaceAccount<'info,Mint>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault : InterfaceAccount<'info,TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_ata_a : InterfaceAccount<'info,TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_ata_b : InterfaceAccount<'info,TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_b : InterfaceAccount<'info,TokenAccount>,

    pub token_program:Interface<'info,TokenInterface>,

    pub associated_token_program : Program<'info,AssociatedToken>,
    pub system_program :Program<'info,System>,
    pub rent : Sysvar<'info,Rent>
}

impl<'info> Take <'info>{
    pub fn take(&mut self,amount_b : u64) -> Result<()>{
        let cpi_program = self.token_program.to_account_info();
        
        let transfer_b_to_maker = TransferChecked{
            from: self.taker_ata_b.to_account_info(),
            to:self.maker_ata_b.to_account_info(),
            mint:self.mint_b.to_account_info(),
            authority:self.taker.to_account_info()
        };

        let cpi_ctx_b = CpiContext::new(cpi_program.clone(), transfer_b_to_maker);
        transfer_checked(cpi_ctx_b, amount_b, self.mint_b.decimals)?;

        let seeds = &[
            b"escrow",
            self.escrow.maker.as_ref(),
            &self.escrow.seed.to_le_bytes(),
            &[self.escrow.bump]
        ];

        let signer_seeds = &[&seeds[..]];

        let transfer_a_to_taker = TransferChecked{
            from:self.vault.to_account_info(),
            to:self.taker_ata_a.to_account_info(),
            mint : self.mint_a.to_account_info(),
            authority: self.escrow.to_account_info()
        };
        let cpi_ctx_a = CpiContext::new_with_signer(cpi_program, transfer_a_to_taker, signer_seeds);
        transfer_checked(cpi_ctx_a, self.escrow.receive, self.mint_a.decimals)?;
        Ok(())
        }
    }