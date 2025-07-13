use anchor_lang::prelude::*;
#[account]
#[derive(InitSpace)]
pub struct User{
    pub seed:u64,
    pub mintusc:Pubkey,
    pub amt:u64,
    pub bump:u8
}
#[account]
#[derive(InitSpace)]
pub struct Initializes{
    pub bump:u8,

}