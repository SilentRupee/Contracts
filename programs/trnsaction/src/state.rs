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
    pub commitment:[u8;32],       // ✅ Fixed typo
    pub nullifier_hash:[u8;32],   // ✅ Fixed typo  
    pub encrypted_balance:[u8;64], // ✅ Fixed typo
    pub shard_id:u64,             // ✅ Fixed typo
    pub created_at: i64,          // ✅ Add timestamp for tracking
    pub bump:u8
}
#[account]
#[derive(InitSpace)]
pub struct IdentityVault{
    pub identity_hash:[u8;32],
    pub encrypted_name:[u8;64],
    pub encrypted_phone:[u8;32],
    pub is_merchant: bool,              // ✅ Use boolean instead of Pubkey
    pub merchant_category: u8,          // ✅ Category (1=restaurant, 2=store, etc.)
    pub identity_commitment:[u8;32],
    pub created_at: i64,                // ✅ Add timestamp
    pub bump:u8
}
// ✅ FIXED: Separate transaction components (unlinkable)
#[account]
#[derive(InitSpace)]
pub struct CustomerTransactionView {
    pub transaction_hash: [u8; 32],          // Hash, not Pubkey
    pub merchant_name_encrypted: [u8; 64],   // Encrypted for customer only
    pub encrypted_amount: [u8; 32],          // Encrypted for customer
    pub timestamp: i64,
    pub customer_nullifier: [u8; 32],       // Only customer's nullifier
    pub bump: u8,
    // ❌ REMOVED: merchant_nullifier (prevents linking)
}

#[account]
#[derive(InitSpace)]
pub struct MerchantTransactionView {
    pub transaction_hash: [u8; 32],          // Same hash, different view
    pub customer_name_encrypted: [u8; 64],   // Encrypted for merchant only
    pub encrypted_amount: [u8; 32],          // Encrypted for merchant
    pub timestamp: i64,
    pub merchant_nullifier: [u8; 32],       // Only merchant's nullifier
    pub bump: u8,
    // ❌ REMOVED: customer_nullifier (prevents linking)
}

// ✅ NEW: Stealth address system
#[account]
#[derive(InitSpace)]
pub struct StealthAddress {
    pub address_commitment: [u8; 32],   // Commitment to stealth address
    pub ephemeral_key: [u8; 32],        // One-time key for this transaction
    pub encrypted_metadata: [u8; 128],  // Encrypted transaction details
    pub nullifier_used: bool,           // Prevents double-spending
    pub timestamp: i64,
    pub bump: u8,
}
#[account]
#[derive(InitSpace)]
pub struct ZkTransaction {
    pub root: [u8; 32],         // ✅ Good
    pub proof_a: [u8; 64],      // ✅ Good
    pub proof_b: [u8; 64],      // ✅ Good  
    pub proof_c: [u8; 32],      // ✅ Good
    
    #[max_len(128)]
    pub encrypted_memo: Vec<u8>, // ✅ Good
    
    pub timestamp: i64,          // ✅ Good
    pub bump: u8                 // ✅ Good
}