use anchor_lang::prelude::*;

declare_id!("Hwcwd77wrNWvAYEfFVJS2ALL2Gr3gJpRUxEFvHTragMr");
pub mod state;
pub mod instructions;
pub use state::*;
pub use instructions::*;


#[program]
pub mod trnsaction {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
