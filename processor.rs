#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::{instruction::Instruction, pubkey::Pubkey, account_info::AccountInfo};
    use solana_program_test::*;
    use solana_sdk::{signature::Signer, transaction::Transaction, transport::TransportError};
    use assert_matches::assert_matches;

    #[tokio::test]
    async fn test_initialize_account_success() {
        // Setup ProgramTest
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::new("your_program", program_id, processor!(process_instruction));

        // Create an account and fund the payer
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Create an instruction for initializing the account
        let instruction = Instruction {
            program_id,
            accounts: vec![/* Your account meta setup */],
            data: vec![0], // Instruction discriminator for `initialize_account`
        };

        // Build and send the transaction
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );

        // Process transaction and assert success
        let result = banks_client.process_transaction(transaction).await;
        assert_matches!(result, Ok(()));
    }

    #[tokio::test]
    async fn test_update_account_balance_success() {
        // Setup ProgramTest
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::new("your_program", program_id, processor!(process_instruction));

        // Setup test client and payer
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Create an account and initialize it
        let instruction = Instruction {
            program_id,
            accounts: vec![/* Account setup */],
            data: vec![0], // Instruction for initializing account
        };

        // Send the transaction
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );
        banks_client.process_transaction(transaction).await.unwrap();

        // Now, update the account balance
        let update_instruction = Instruction {
            program_id,
            accounts: vec![/* Same account as before */],
            data: vec![1, 100], // Instruction discriminator for `update_account`, and balance update data (100)
        };

        let update_transaction = Transaction::new_signed_with_payer(
            &[update_instruction],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );

        let result = banks_client.process_transaction(update_transaction).await;
        assert_matches!(result, Ok(()));
    }

    #[tokio::test]
    async fn test_invalid_instruction_data() {
        // Setup ProgramTest
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::new("your_program", program_id, processor!(process_instruction));

        // Setup test client and payer
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Create instruction with invalid data
        let invalid_instruction = Instruction {
            program_id,
            accounts: vec![/* Your accounts setup here */],
            data: vec![255], // Invalid instruction discriminator
        };

        // Send the transaction
        let transaction = Transaction::new_signed_with_payer(
            &[invalid_instruction],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );

        // This should result in an error
        let result = banks_client.process_transaction(transaction).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_missing_account() {
        // Setup ProgramTest
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::new("your_program", program_id, processor!(process_instruction));

        // Setup test client and payer
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Create instruction without providing required accounts
        let missing_account_instruction = Instruction {
            program_id,
            accounts: vec![], // Missing required account
            data: vec![0], // Assuming it's an initialize instruction
        };

        // Send the transaction
        let transaction = Transaction::new_signed_with_payer(
            &[missing_account_instruction],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );

        // This should result in an error
        let result = banks_client.process_transaction(transaction).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_insufficient_funds() {
        // Setup ProgramTest
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::new("your_program", program_id, processor!(process_instruction));

        // Setup test client and payer
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Create instruction to simulate insufficient funds
        let instruction = Instruction {
            program_id,
            accounts: vec![/* Account setup with low balance */],
            data: vec![2], // Instruction that requires more funds than available
        };

        // Send the transaction
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );

        // Expect failure due to insufficient funds
        let result = banks_client.process_transaction(transaction).await;
        assert!(result.is_err());
    }
}
