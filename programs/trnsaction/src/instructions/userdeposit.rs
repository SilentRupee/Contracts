use anchor_lang::{accounts::program, prelude::*, system_program::Transfer};
use anchor_spl::{associated_token::AssociatedToken, token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked}};
use crate::{state::Initializes, User};
#[derive(Accounts)]
pub struct Userdeposit<'info>{
    #[account(mut)]
    pub signer:Signer<'info>,
    #[account(mut)]
    pub owner:SystemAccount<'info>,
    pub  mint:Account<'info,Mint>,
    #[account(mut,associated_token::mint=mint,associated_token::authority=signer,associated_token::token_program=token_program)]
    pub user_wallet_usdc:Account<'info,TokenAccount>,
    #[account(mut,seeds=[b"pool",owner.key().as_ref()],bump=vault.bump)]
    pub vault:Account<'info,Initializes>,
    #[account(mut,seeds=[b"escrow",signer.key().as_ref(),escrow.seed.to_le_bytes().as_ref()],bump=escrow.bump)]
    pub escrow:Account<'info,User>,
    #[account(mut,associated_token::authority=escrow,associated_token::token_program=token_program,associated_token::mint=mint)]
    pub user_usdc:Account<'info,TokenAccount>,
    pub system_program:Program<'info,System>,
     pub token_program:Program<'info,Token>,
     pub associated_token_program:Program<'info,AssociatedToken>
}
impl<'info> Userdeposit<'info>{
    pub fn tranasfer(&mut self,amount:u64) -> Result<()>{
         let account=TransferChecked{
            from:self.user_wallet_usdc.to_account_info(),
            to:self.user_usdc.to_account_info(),
            authority:self.signer.to_account_info(),
            mint:self.mint.to_account_info()
         };
         let cpi_context=CpiContext::new(self.token_program.to_account_info(), account);
         transfer_checked(cpi_context, amount, self.mint.decimals)?;

        Ok(())
    } 
} 