use anchor_lang::{prelude::*, solana_program::example_mocks::solana_transaction::Transaction};
#[account]
#[derive(InitSpace)]
pub struct User{
    pub seed:u64,
    pub mintusc:Pubkey,
    pub bump:u8,
}
#[account]
#[derive(InitSpace)]
pub struct Initializes{
    pub bump:u8,
    pub usdc:Pubkey,
    pub total_user:u64,
    pub total_transactions:u64
}
#[account]
#[derive(InitSpace)]
pub struct EncryptedAccount{
    pub commitment:[u8;32],      
    pub nullifier_hash:[u8;32],   
    pub encrypted_balance:[u8;64], 
    pub shard_id:u64,            
    pub created_at: i64,    
    pub encrypted_salt: [u8;32],   
    pub bump:u8
}

#[account]
#[derive(InitSpace)]
pub struct StealthAddress {
    pub address_commitment: [u8; 32],  
    pub ephemeral_key: [u8; 32],        
    pub encrypted_metadata: [u8; 128],  
    pub nullifier_used: bool,          
    pub timestamp: i64,
    pub bump: u8,
}
#[account]
#[derive(InitSpace)]
pub struct ZkTransaction {
    pub root: [u8; 32],         
    pub proof_a: [u8; 64],      
    pub proof_b: [u8; 64],      
    pub proof_c: [u8; 32],      
    #[max_len(128)]
    pub encrypted_memo: Vec<u8>,
    pub timestamp: i64,          
    pub bump: u8                
}