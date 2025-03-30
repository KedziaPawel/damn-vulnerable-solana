use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Wallet {
    pub authority: Pubkey,
}

pub const WALLET_LEN: u64 = 32 + 32;
