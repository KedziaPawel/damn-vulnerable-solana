use {
    borsh::BorshSerialize,
    solana_program::{pubkey::Pubkey, system_program},
    solana_program_test::*,
    solana_sdk::{
        account::Account,
        hash::Hash,
        signature::{Keypair, Signer},
    },
};

use wallet_what_wallet::state::{Wallet, WALLET_LEN};

#[allow(clippy::too_many_arguments)]
async fn hack(
    _banks_client: &mut BanksClient,
    _payer: &Keypair,
    _recent_blockhash: Hash,
    _program_id: &Pubkey,
    _wallet_address: Pubkey,
    _wallet_authority: &Pubkey,
    _destination: &Pubkey,
    _amount: u64,
) -> Result<(), BanksClientError> {
    todo!()
}

#[tokio::test]
async fn test_hack() {
    // Initialize program test
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "wallet-what-wallet",
        program_id,
        processor!(wallet_what_wallet::processor::process_instruction),
    );

    // Create test accounts
    let wallet_authority = Keypair::new();
    let destination = Keypair::new();

    // Calculate the wallet address
    let (wallet_address, _) =
        Pubkey::find_program_address(&[&wallet_authority.pubkey().to_bytes()], &program_id);

    // Add wallet_authority account with lamports
    program_test.add_account(
        wallet_authority.pubkey(),
        Account::new(10_000_000_000, 0, &system_program::id()),
    );

    // Add destination account with minimum rent-exempt balance
    let destination_initial_balance = 890880; // 0.00089088 SOL - minimum rent for an empty account
    program_test.add_account(
        destination.pubkey(),
        Account::new(destination_initial_balance, 0, &system_program::id()),
    );

    // Create wallet data
    let wallet = Wallet {
        authority: wallet_authority.pubkey(),
    };
    let mut wallet_data = vec![0; WALLET_LEN as usize];
    wallet.serialize(&mut wallet_data.as_mut_slice()).unwrap();

    // Add the wallet account with initial funds
    let initial_funds = 1_000_000;
    program_test.add_account(
        wallet_address,
        Account {
            lamports: initial_funds,
            data: wallet_data,
            owner: program_id,
            executable: false,
            rent_epoch: 0,
        },
    );

    // Start program test
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Execute the hack to drain the wallet
    hack(
        &mut banks_client,
        &payer,
        recent_blockhash,
        &program_id,
        wallet_address,
        &wallet_authority.pubkey(),
        &destination.pubkey(),
        initial_funds, // Withdraw the entire balance
    )
    .await
    .unwrap();

    // Verify the hack worked - funds moved to destination
    let destination_account = banks_client
        .get_account(destination.pubkey())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        destination_account.lamports,
        destination_initial_balance + initial_funds
    );

    // Verify the wallet account is gone (this is expected behavior in Solana when an account's balance is zero)
    let wallet_account = banks_client.get_account(wallet_address).await.unwrap();
    assert!(
        wallet_account.is_none(),
        "Wallet account should no longer exist after being drained"
    );
}
