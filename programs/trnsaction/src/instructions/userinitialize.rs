use anchor_lang::{accounts::program, prelude::*};
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};
use crate::{state::Initializes, User};
#[derive(Accounts)]
#[instruction(seeds:u64)]
pub struct Userinitialize<'info>{
    #[account(mut)]
    pub signer:Signer<'info>,
    #[account(mut)]
    pub owner:SystemAccount<'info>,
    #[account(init,payer=signer,seeds=[b"pool",signer.key().as_ref()],bump,space=8+ Initializes::INIT_SPACE)]
    pub vault:Account<'info,Initializes>,
    pub mint:Account<'info,Mint>,
    #[account(init,payer=signer,associated_token::mint=mint,associated_token::token_program=token_program,associated_token::authority=vault)]
    pub usdc_ata:Account<'info,TokenAccount>,
    pub system_program:Program<'info,System>,
    pub token_program:Program<'info,Token>,
    pub associated_token_program:Program<'info,AssociatedToken>
}
impl<'info> Userinitialize<'info>{
    pub fn initialize(&mut self,seeds:u64,bump:&UserinitializeBumps) -> Result<()>{
         self.escrow.bump=bump.escrow;
         self.escrow.seed=seeds;
         self.escrow.mintusc=self.mint.key();
        Ok(())
    } 
} 