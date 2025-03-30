use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub enum WalletInstruction {
    Initialize,
    Deposit { amount: u64 },
    Withdraw { amount: u64 },
}
