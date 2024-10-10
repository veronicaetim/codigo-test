#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_error_message() {
        let error = CustomError::InsufficientFunds;
        assert_eq!(error.to_string(), "Insufficient Funds");

        let result: Result<(), ProgramError> = Err(error.into());
        assert_matches!(result, Err(ProgramError::Custom(0)));
    }

    #[test]
    fn test_invalid_account_owner_error() {
        let error = CustomError::InvalidAccountOwner;
        assert_eq!(error.to_string(), "Invalid Account Owner");

        let result: Result<(), ProgramError> = Err(error.into());
        assert!(result.is_err());
    }

    #[test]
    fn test_error_handling_edge_cases() {
        let error = CustomError::Overflow;
        assert_eq!(error.to_string(), "Arithmetic Overflow");

        let result: Result<(), ProgramError> = Err(error.into());
        assert!(result.is_err());
    }
}
