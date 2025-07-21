use std::slice;

use anchor_lang::{accounts::program, prelude::*};
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};
use crate::{state::Initializes, EncryptedAccount, User};
#[derive(Accounts)]
#[instruction(user_seed: u64, encrypted_salt: [u8; 32], identity_salt: [u8; 32])]
pub struct Userinitialize<'info>{
    #[account(mut)]
    pub signer:Signer<'info>,
    #[account(mut)]
    pub owner:SystemAccount<'info>,
    #[account(mut,seeds=[b"pool",owner.key().as_ref()],bump=vault.bump)]
    pub vault:Account<'info,Initializes>,
    pub mint:Account<'info,Mint>,
    #[account(mut,associated_token::mint=mint,associated_token::token_program=token_program,associated_token::authority=vault)]
    pub vault_ata:Account<'info,TokenAccount>,
    #[account(init,seeds=[b"user",signer.key().as_ref(),user_seed.to_le_bytes().as_ref()],payer=signer,bump,space=8+User::INIT_SPACE)]
    pub user:Account<'info,User>,
    #[account(init,payer=signer,seeds=[b"encrpted",encrypted_salt.as_ref(),user_seed.to_le_bytes().as_ref()],bump,space=8+EncryptedAccount::INIT_SPACE)]
     pub encryted_account:Account<'info,EncryptedAccount>,   
    pub system_program:Program<'info,System>,
    pub token_program:Program<'info,Token>,
    pub associated_token_program:Program<'info,AssociatedToken>
}
impl<'info> Userinitialize<'info>{
    pub fn initialize(&mut self,bump:&UserinitializeBumps,user_Seed:u64,encrypted_salt:[u8;32],identity_salt:[u8;32],encrypted_balance:[u8;64],nullifier:[u8;32],commitemnt:[u8;32]) -> Result<()>{
          self.user.seed=user_Seed;
          self.user.bump=bump.user;
          self.user.mintusc=self.mint.key();
          self.encryted_account.commitment=commitemnt;
          self.encryted_account.nullifier_hash=generate_nullifer_hash(&nullifier,&encrypted_salt);
          self.encryted_account.encrypted_salt=encrypted_salt;
          self.encryted_account.encrypted_balance=encrypted_balance;
          self.encryted_account.shard_id=generate_random_shard(&encrypted_salt);
          self.encryted_account.created_at=Clock::get()?.unix_timestamp;
          self.encryted_account.bump=bump.encryted_account;
        self.vault.total_user+=1;
        

        Ok(())
    } 
} 
fn generate_nullifer_hash(nullifier:&[u8;32],salt:&[u8;32])->[u8;32]{
    use anchor_lang::solana_program::keccak;//unique identifier for a transaction without revealing the actual transaction details
    let mut data=Vec::new();
    data.extend_from_slice(nullifier);
    data.extend_from_slice(salt);
    data.extend_from_slice(b"nullifier_hash");
    keccak::hash(&data).to_bytes()
}
fn generate_random_shard(salt:&[u8;32])-> u64{
   use anchor_lang::solana_program::keccak;
   let hash=keccak::hash(salt);
   u64::from_le_bytes([hash.0[0],hash.0[1],hash.0[2],hash.0[3],hash.0[4],hash.0[5],hash.0[6],hash.0[7]])%100
}
fn array_from_slice(slice:&[u8])->[u8;64]{
    let mut array=[0u8;64];
    array[..slice.len().min(64)].copy_from_slice(&slice[..slice.len().min(64)]);
    array
}