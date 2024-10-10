#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::pubkey::Pubkey;

    #[test]
    fn test_initialize_account() {
        let account = Account::default();
        assert_eq!(account.balance, 0);
        assert_eq!(account.owner, Pubkey::default());
    }

    #[test]
    fn test_update_account_balance() {
        let mut account = Account::default();
        account.balance += 100;
        assert_eq!(account.balance, 100);
    }

    #[test]
    fn test_initialize_pda() {
        let pda = AccountPDA::new(Pubkey::default(), 100);
        assert_eq!(pda.balance, 100);
        assert_eq!(pda.owner, Pubkey::default());
    }

    #[test]
    fn test_account_state_modification() {
        let mut account = Account::default();
        account.balance += 50;
        account.balance -= 25;
        assert_eq!(account.balance, 25);
    }

    #[test]
    fn test_pda_owner_update() {
        let mut pda = AccountPDA::new(Pubkey::default(), 100);
        let new_owner = Pubkey::new_unique();
        pda.owner = new_owner;
        assert_eq!(pda.owner, new_owner);
    }

    #[test]
    fn test_max_balance() {
        let mut account = Account::default();
        account.balance = u64::MAX;
        assert_eq!(account.balance, u64::MAX);
    }
}
