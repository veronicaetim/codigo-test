#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_initialize_account() {
        let data = vec![0]; // Assuming 0 is the discriminator for `initialize_account`
        let instruction = parse_instruction_data(&data).unwrap();
        match instruction {
            YourInstruction::InitializeAccount => {}, // Successfully parsed
            _ => panic!("Failed to parse initialize account"),
        }
    }

    #[test]
    fn test_parse_valid_update_account() {
        let data = vec![1, 100]; // Discriminator and balance to update (100)
        let instruction = parse_instruction_data(&data).unwrap();
        match instruction {
            YourInstruction::UpdateAccount { balance } => {
                assert_eq!(balance, 100); // Ensure the correct balance was parsed
            }
            _ => panic!("Failed to parse update account"),
        }
    }

    #[test]
    fn test_parse_invalid_instruction() {
        let data = vec![255]; // Invalid discriminator
        let result = parse_instruction_data(&data);
        assert!(result.is_err()); // Expect error due to invalid discriminator
    }

    #[test]
    fn test_parse_edge_cases() {
        // Test edge cases like empty data, large numbers, etc.
        let empty_data = vec![];
        assert!(parse_instruction_data(&empty_data).is_err());

        let large_data = vec![1, u8::MAX];
        let result = parse_instruction_data(&large_data).unwrap();
        match result {
            YourInstruction::UpdateAccount { balance } => assert_eq!(balance, u8::MAX),
            _ => panic!("Failed to parse large balance value"),
        }
    }
}
