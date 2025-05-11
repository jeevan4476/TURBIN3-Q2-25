use anchor_lang::{prelude::*,system_program::{transfer,Transfer}};
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{ MasterEditionAccount, Metadata, MetadataAccount},
    token_interface::{Mint,TokenAccount,TokenInterface}
};

use crate::state::{Listing,Marketplace};
#[derive(Accounts)]
pub struct Purchase<'info>{
    #[account(mut)]
    pub taker: Signer<'info>,

    pub maker : SystemAccount<'info>,

    #[account(
        seeds = [b"Marketplace",marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump
    )]
    pub marketplace : Account<'info,Marketplace>,
    
    pub maker_mint: InterfaceAccount<'info,Mint>,

    #[account{
        init_if_needed,
        payer=taker,
        associated_token::mint = maker_mint,
        associated_token::authority= taker
    }]
    pub taker_ata : InterfaceAccount<'info,TokenAccount>,

    #[account{
        init_if_needed,
        payer=taker,
        associated_token::mint = reward_mint,
        associated_token::authority= taker
    }]
    pub taker_rewards_ata : InterfaceAccount<'info,TokenAccount>,
    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority=listing,
    )]
    pub vault: InterfaceAccount<'info,TokenAccount>,

    #[account(
        mut,
        close = maker,
        seeds = [marketplace.key().as_ref(),maker_mint.key().as_ref()],
        bump = listing.bump,
    )]
    pub listing : Account<'info,Listing>,

    #[account(
        seeds=[b"treasury",marketplace.key().as_ref()],
        bump
    )]
    pub treasury: SystemAccount<'info>,
    
    #[account(
        mut,
        seeds = [ b"rewards",marketplace.key().as_ref()],
        bump = marketplace.rewards_bump,
        mint::decimals = 6,
        mint::authority = marketplace
    )]
    pub reward_mint : InterfaceAccount<'info,Mint>,


    pub collection_mint : InterfaceAccount<'info,Mint>,
    pub metadata_program : Program<'info,Metadata>,
    pub associated_token_program : Program<'info,AssociatedToken>,
    pub system_program : Program<'info,System>,
    pub token_program : Interface<'info,TokenInterface>,
}

impl <'info> Purchase<'info>{
    pub fn send_sol(&mut self)->Result<()>{

        let marketplace_fee = (self.marketplace.fee as u64)
        .checked_mul(self.listing.price)
        .unwrap()
        .checked_div(10000_u64)
        .unwrap();
        
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer{
            from : self.taker.to_account_info(),
            to:self.maker.to_account_info(),
        };
        
        let cpi_ctx = CpiContext::new(cpi_program.clone(), cpi_accounts);

        let amount = self.listing.price.checked_sub(marketplace_fee).unwrap();
        transfer(cpi_ctx, amount)?;


        let cpi_accounts = Transfer{
            from : self.taker.to_account_info(),
            to:self.treasury.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program.clone(), cpi_accounts);

        transfer(cpi_ctx, marketplace_fee)?; 
        Ok(())
    }

    
}