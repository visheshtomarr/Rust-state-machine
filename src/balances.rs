use std::collections::BTreeMap ; 

/// This is the Balances module.
/// It is a simple module that keeps track of how much balance a user has in our state machine.
pub struct Pallet {
    balances: BTreeMap<String, u128>,
}

impl Pallet {
    /// Create a new instance of our balances module.
    pub fn new(&self) -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    /// Set the balance of an account "who" to some "amount".
    pub fn set_balance(&mut self, who: &String, amount: u128) {
        self.balances.insert(who.clone(), amount) ;
    }

    /// Get the balance of an account "who".
    /// If the account has no stored balance, we return zero.
    pub fn balance(&self, who: &String) -> u128 {
        *self.balances.get(who).unwrap_or(&0) 
    }
}