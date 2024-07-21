use std::collections::BTreeMap ;

/// This is the system Pallet.
/// It handles low level state needed for our blockchain.
#[derive(Debug)]
pub struct Pallet {
    /// The current block number.
    block_number: u32,
    /// A map from an account to their "nonce".
    nonce: BTreeMap<String, u32>,
}

impl Pallet {
    /// Create a new instance of the System pallet.
    pub fn new() -> Self {
        Self {
            block_number: 0,
            nonce: BTreeMap::new() ,
        }
    }

    /// Get the current block number.
    pub fn block_number(&self) -> u32 {
        self.block_number
    }

    /// Increment the block number by one.
    pub fn inc_block_number(&mut self) {
        self.block_number += 1 ;
    }

    /// Increment the nonce of an account. This helps us keep track of how many transactions
    /// each account has made.
    pub fn inc_nonce(&mut self, who: &String) {
        let nonce = *self.nonce.get(who).unwrap_or(&0) ;
        let new_nonce = nonce + 1  ;
        self.nonce.insert(who.clone(), new_nonce) ;
    }
}

#[cfg(test)]
mod test {
    #[test] 
    fn init_system() {
        // Instantiating a system struct.
        let mut system = crate::system::Pallet::new() ;

        // Increment the current block number.
        system.inc_block_number() ;

        // Increment nonce of "alice".
        system.inc_nonce(&"alice".to_string()) ;

        // Assert block number is updated or not.
        assert_eq!(system.block_number, 1) ;

        // Assert nonce of "alice" is updated or not.
        assert_eq!(system.nonce.get(&"alice".to_string()), Some(&1)) ;

        // Assert nonce of "bob" is none.
        assert_eq!(system.nonce.get(&"bob".to_string()), None) ;
    }
}