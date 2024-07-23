use std::collections::BTreeMap ; 

type AccountId = String ;
type Balance = u128 ;

/// This is the Balances module.
/// It is a simple module that keeps track of how much balance a user has in our state machine.
#[derive(Debug)]
pub struct Pallet {
    balances: BTreeMap<AccountId, Balance>,
}

impl Pallet {
    /// Create a new instance of our balances module.
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    /// Set the balance of an account "who" to some "amount".
    pub fn set_balance(&mut self, who: &AccountId, amount: Balance) {
        self.balances.insert(who.clone(), amount) ;
    }

    /// Get the balance of an account "who".
    /// If the account has no stored balance, we return zero.
    pub fn balance(&self, who: &AccountId) -> Balance {
        *self.balances.get(who).unwrap_or(&0) 
    }

    /// Transfer some "amount" from one account to another.
    /// This function verifies that "from" has atleast "amount" balance to transfer and that no
    /// mathematical overflow occurs.
    pub fn transfer(
        &mut self, 
        caller: AccountId,
        to: AccountId,
        amount: Balance
    ) -> Result<(), &'static str> {

        // Get balance of both user pre-transfer.
        let caller_balance = self.balance(&caller) ;
        let to_balance = self.balance(&to) ;

        // Calculate new balances of both "caller" & "to" accounts while keeping check of underflow and overflow.
        let new_caller_balance = caller_balance.checked_sub(amount).ok_or("Insufficient funds.") ?;
        let new_to_balance = to_balance.checked_add(amount).ok_or("Overflow.") ?;

        // Update balances of both accounts post-transfer.
        self.balances.insert(caller, new_caller_balance) ;
        self.balances.insert(to, new_to_balance) ;

        Ok(()) 
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn init_balances(){
        // Instantiating a balances struct.
        let mut balances = crate::balances::Pallet::new();

        // Assert that the balance of "alice" starts at zero. 
        assert_eq!(balances.balance(&"alice".to_string()), 0) ;
        // Set balance of "alice" to 100.
        balances.set_balance(&"alice".to_string(), 100) ;
        // Assert that "alice" has now balance of 100.
        assert_eq!(balances.balance(&"alice".to_string()), 100) ;
        // Assert balance of "bob" has not changed and is equal to zero.
        assert_eq!(balances.balance(&"bob".to_string()), 0) ;
    }

    #[test]
    fn transfer_balance() {
        // Instantiating a balances struct
        let mut balances = crate::balances::Pallet::new() ;
        
        // Alice cannot transfer funds she doesn't have.
        assert_eq!(
            balances.transfer("alice".to_string(), "bob".to_string(), 50),
            Err("Insufficient funds.")
        ) ;

        // Providing alice with some balance.
        balances.set_balance(&"alice".to_string(), 100) ;

        // Alice can now transfer funds.
        assert_eq!(
            balances.transfer("alice".to_string(), "bob".to_string(), 55),
            Ok(())
        ) ;

        // Check both accounts' balances updated successfully.
        assert_eq!(balances.balance(&"alice".to_string()), 45) ;
        assert_eq!(balances.balance(&"bob".to_string()), 55) ;

        // Alice can no longer transfer funds greater than amount of 45.
        assert_eq!(
            balances.transfer("alice".to_string(), "bob".to_string(), 50),
            Err("Insufficient funds.")
        ) ;
    }
}