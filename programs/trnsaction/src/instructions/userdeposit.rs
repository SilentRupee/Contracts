use anchor_lang::{accounts::program, prelude::*, system_program::Transfer};
use anchor_spl::{associated_token::AssociatedToken, token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked}};
use crate::{state::Initializes, EncryptedAccount, User,ZkTransaction};
#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct Userdeposit<'info>{
    #[account(mut)]
    pub signer:Signer<'info>,
    #[account(mut)]
    pub owner:SystemAccount<'info>,
    #[account(mut,seeds=[b"pool",owner.key().as_ref()],bump=vault.bump)]
    pub vault:Account<'info,Initializes>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub user_external_wallet: Account<'info, TokenAccount>,
    
    pub mint:Account<'info,Mint>,
    #[account(mut,associated_token::mint=mint,associated_token::token_program=token_program,associated_token::authority=vault)]
    pub vault_ata:Account<'info,TokenAccount>,
    #[account(seeds=[b"user",signer.key().as_ref(),],bump=user.seed)]
    pub user:Account<'info,User>,
    #[account(seeds=[b"encrpted",encryted_account.encrypted_salt.as_ref(),user.seed.to_le_bytes().as_ref()],bump=encryted_account.bump)]
     pub encryted_account:Account<'info,EncryptedAccount>,   
     #[account(init,payer=signer,seeds=[b"zkproof",encryted_account.encrypted_salt.as_ref(),amount.to_le_bytes().as_ref()],bump,space=8+ ZkTransaction::INIT_SPACE)]
     pub zkaccount:Account<'info,ZkTransaction>,
    pub system_program:Program<'info,System>,
    pub token_program:Program<'info,Token>,
    pub associated_token_program:Program<'info,AssociatedToken>
}
impl<'info> Userdeposit<'info>{
    pub fn tranasfer(&mut self,amount:u64,zk_proof:[u8;256],bump:UserdepositBumps) -> Result<()>{
        require!(
            self.verify_deposit_zk_proof(&zk_proof, amount,&self.encryted_account.commitment,&self.encryted_account.encrypted_balance)?,
            ErrorCode::InvalidZkProof
        );
        transfer_checked(
            CpiContext::new(
                self.token_program.to_account_info(),
                TransferChecked {
                    from: self.user_external_wallet.to_account_info(),
                    mint: self.mint.to_account_info(),
                    to: self.vault_ata.to_account_info(),
                    authority: self.user.to_account_info(),
                },
            ),
            amount,
            6, // USDC decimals
        )?;
        self.zkaccount.root = self.generate_merkle_root(self.encryted_account.commitment)?;
        self.zkaccount.proof_a = array_from_slice(&zk_proof[0..64]);
        self.zkaccount.proof_b = array_from_slice(&zk_proof[64..128]);
        self.zkaccount.proof_c = array_from_slice(&zk_proof[128..160]);
        self.zkaccount.encrypted_memo = vec![]; // Can store additional encrypted data
        self.zkaccount.timestamp = Clock::get()?.unix_timestamp;
        self.zkaccount.bump =bump.zkaccount;

        Ok(())
    } 
    fn verify_deposit_zk_proof(
        &self,
        proof: &[u8; 256],
        amount: u64,
        commitment: &[u8; 32],
        encrypted_balance: &[u8; 64],
    ) -> Result<bool> {
        // In production, this would use proper ZK verification
        // For now, implementing simplified verification
        
        // Verify proof structure is valid
        require!(proof.len() == 256, ErrorCode::InvalidProofLength);
        
        // Verify commitment matches expected format
        let expected_commitment = self.generate_commitment(amount, encrypted_balance)?;
        require!(commitment == &expected_commitment, ErrorCode::InvalidCommitment);
        
        // Verify encrypted balance corresponds to deposit amount
        // (In practice, this would be verified through ZK proof)
        require!(self.verify_encrypted_balance(amount, encrypted_balance)?, ErrorCode::InvalidEncryptedBalance);
        
        // TODO: Add actual Groth16/PLONK verification here
        // For MVP, accept valid proof structure
        Ok(true)
    }
    fn verify_encrypted_balance(&self, amount: u64, encrypted_balance: &[u8; 64]) -> Result<bool> {
        // Simplified verification - in production would be part of ZK proof
        // Check that encrypted balance is not all zeros (indicates proper encryption)
        let non_zero = encrypted_balance.iter().any(|&x| x != 0);
        Ok(non_zero && amount > 0)
    }
    fn generate_commitment(&self, amount: u64, encrypted_balance: &[u8; 64]) -> Result<[u8; 32]> {
        use anchor_lang::solana_program::keccak;
        
        let mut data = Vec::new();
        data.extend_from_slice(&amount.to_le_bytes());
        data.extend_from_slice(encrypted_balance);
        data.extend_from_slice(&self.user.key().to_bytes());
        data.extend_from_slice(b"deposit_commitment");
        
        Ok(keccak::hash(&data).to_bytes())
    }
    fn generate_merkle_root(&self, commitment: [u8; 32]) -> Result<[u8; 32]> {
        // For MVP, use commitment as root
        // In production, maintain proper Merkle tree
        Ok(commitment)
    }
    
}
 

fn array_from_slice(slice: &[u8]) -> [u8; 64] {
    let mut array = [0u8; 64];
    array[..slice.len().min(64)].copy_from_slice(&slice[..slice.len().min(64)]);
    array
}
// Error codes
#[error_code]
pub enum ErrorCode {
    #[msg("Invalid ZK proof provided")]
    InvalidZkProof,
    #[msg("Invalid proof length")]
    InvalidProofLength,
    #[msg("Invalid commitment")]
    InvalidCommitment,
    #[msg("Invalid encrypted balance")]
    InvalidEncryptedBalance,
}