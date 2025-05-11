use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken, token_interface::{
        transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked
    }};

use crate::state::Escrow;


#[derive(Accounts)]
pub struct Close<'info>{
    #[account(mut)]
    pub maker : Signer<'info>,
    #[account(
        mint::token_program=token_program
    )]
    pub mint_a:InterfaceAccount<'info,Mint>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program= token_program

    )]
    pub maker_ata_a : InterfaceAccount<'info,TokenAccount>,

    #[account(
        mut,
        close = maker,
        has_one = mint_a,
        has_one = maker,
        seeds=[b"escrow",maker.key().as_ref(),escrow.seed.to_le_bytes().as_ref()],
        bump=escrow.bump,
    )]
    pub escrow: Account<'info,Escrow>,

    #[account(
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program= token_program
    )]
    pub vault : InterfaceAccount<'info,TokenAccount>,

    pub token_program:Interface<'info,TokenInterface>,
    pub associated_token_program : Program<'info,AssociatedToken>,
    pub system_program :Program<'info,System>
}

impl<'info> Close<'info>{
    pub fn  close(&mut self) -> Result<()>{

        let cpi_program = self.token_program.to_account_info();
        
        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.maker_ata_a.to_account_info(),
            mint: self.mint_a.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let seeds = [
            b"escrow",
            self.escrow.maker.as_ref(),
            &self.escrow.seed.to_le_bytes(),
            &[self.escrow.bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            cpi_program,cpi_accounts,signer_seeds,
        );

        transfer_checked(cpi_ctx, self.escrow.receive, self.mint_a.decimals)?;
        Ok(())
    }
    
}