use anchor_lang::{accounts::program, prelude::*};
use anchor_spl::token::Mint;
use crate::{state::Initializes};
#[derive(Accounts)]

pub struct Initialize<'info>{
    #[account(mut)]
    pub signer:Signer<'info>,
    #[account(init,payer=signer,seeds=[b"pool",signer.key().as_ref()],bump,space=8+ Initializes::INIT_SPACE)]
    pub vault:Account<'info,Initializes>,

    pub system_program:Program<'info,System>,
}
impl<'info> Initialize<'info>{
    pub fn initialize(&mut self, ctx: &Context<Initialize>) -> Result<()>{
        self.vault.bump = ctx.bumps.vault;
   
        Ok(())
    } 
} 