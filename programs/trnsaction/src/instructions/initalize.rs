use anchor_lang::{accounts::program, prelude::*};
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};
use sha2::digest::typenum::Min;
use crate::{state::Initializes};
#[derive(Accounts)]

pub struct Initialize<'info>{
    #[account(mut)]
    pub signer:Signer<'info>,
    #[account(init,payer=signer,seeds=[b"pool",signer.key().as_ref()],bump,space=8+ Initializes::INIT_SPACE)]
    pub vault:Account<'info,Initializes>,
    pub mint:Account<'info,Mint>,
    #[account(init,payer=signer,associated_token::mint=mint,associated_token::token_program=token_program,associated_token::authority=vault)]
    pub vault_ata:Account<'info,TokenAccount>,
    pub system_program:Program<'info,System>,
    pub token_program:Program<'info,Token>,
    pub associated_token_program:Program<'info,AssociatedToken>
}
impl<'info> Initialize<'info>{
    pub fn initialize(&mut self, ctx: &Context<Initialize>) -> Result<()>{
        self.vault.bump = ctx.bumps.vault;
        self.vault.usdc=self.mint.key();
        Ok(())
    } 
} 