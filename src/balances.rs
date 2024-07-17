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
}