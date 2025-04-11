
use anchor_lang::{prelude::*,system_program::{transfer,Transfer}};

declare_id!("AckRwpcQ3MoGLKESm1tbFbpvqHbbmJ2ANeDeHGKxa1p4");

#[program]
pub mod vault_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)
    }

    pub fn deposit(ctx: Context<Payment>,amount:u64)->Result<()>{
        ctx.accounts.deposit(amount)?;
        Ok(())
    }
    
}

#[derive(Accounts)]
pub struct Initialize <'info>{
    #[account(mut)]
    pub user :Signer<'info>,

    #[account(
        init,
        payer=user,
        space =  8 +   VaultState::INIT_SPACE,
        seeds= [b"state",user.key().as_ref()],
        bump,
    )]
    pub state :Account<'info , VaultState>,

    #[account(
        seeds=[b"vault",state.key().as_ref()],
        bump
    )]
    pub vault : SystemAccount<'info>,

    pub system_program : Program<'info,System>
}


impl<'info> Initialize<'info>{
    pub fn initialize(&mut self,bumps: &InitializeBumps) -> Result<()>{
        self.state.vault_bump = bumps.vault;
        self.state.state_bump = bumps.state;
        self.state.amount = 0;
        Ok(())
    }

}

#[derive(Accounts)]
pub struct Payment<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        seeds = [b"state", signer.key().as_ref()],
        bump = vault_state.state_bump,
    )]
    pub vault_state: Account<'info, VaultState>,
    #[account(
        mut, 
        seeds=[vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,

}

impl<'info> Payment<'info> {
    pub fn deposit(&mut self,amount: u64) -> Result<()> {
        let system_program = self.system_program.to_account_info();
        let accounts = Transfer {
            from: self.signer.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_cpx = CpiContext::new(system_program, accounts);

        transfer(cpi_cpx, amount)?;
        
        Ok(())
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer{
            from: self.vault_state.to_account_info(),
            to: self.signer.to_account_info(),
        };

        let seeds = &[
            b"state",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(cpi_ctx, amount)?;

        Ok(())
    }
}
#[account]
#[derive(InitSpace)]
pub struct  VaultState{
    pub vault_bump : u8, //for vault
    pub state_bump: u8  , //for pda 
    pub amount : u64
}