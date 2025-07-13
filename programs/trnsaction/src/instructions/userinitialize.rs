use anchor_lang::{accounts::program, prelude::*};
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};
use crate::{state::Initializes, User};
#[derive(Accounts)]
#[instruction(seeds:u64)]
pub struct Userinitialize<'info>{
    #[account(mut)]
    pub signer:Signer<'info>,
    pub owner:SystemAccount<'info>,
    pub  mint:Account<'info,Mint>,
    #[account(seeds=[b"pool",owner.key().as_ref()],bump=vault.bump)]
    pub vault:Account<'info,Initializes>,
    #[account(init,payer=signer,seeds=[b"escrow",signer.key().as_ref()],bump,space=User::INIT_SPACE)]
    pub escrow:Account<'info,User>,
    #[account(init,associated_token::authority=escrow,associated_token::token_program=token_program,associated_token::mint=mint,payer=signer)]
    pub user_usdc:Account<'info,TokenAccount>,
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