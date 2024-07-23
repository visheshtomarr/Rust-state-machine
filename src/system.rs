use num::traits::{Zero, One} ; 
use std::collections::BTreeMap ;
use core::ops::AddAssign ;

/// The Config trait for the System module.
/// It contains the types AccountId, BlockNumber and Nonce, which is a BTreeMap from an account to their nonce. 
pub trait Config {
    /// A type to identify account in our state machine.
    /// On a real blockchain, we would want this to be a cryptgraphic public key.
    type AccountId: Ord + Clone ;
    /// A type to identify the current block number.
    type BlockNumber: Zero + One + Copy + AddAssign ;
    /// A type to keep count of the transactions a particular user has done.
    type Nonce: Zero + One + Copy ;
}

/// This is the system Pallet.
/// It handles low level state needed for our blockchain.
#[derive(Debug)]
pub struct Pallet<T: Config> {
    /// The current block number.
    block_number: T::BlockNumber,
    /// A map from an account to their "nonce".
    nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
    /// Create a new instance of the System pallet.
    pub fn new() -> Self {
        Self {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new() ,
        }
    }

    /// Get the current block number.
    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    /// Increment the block number by one.
    pub fn inc_block_number(&mut self) {
        self.block_number += T::BlockNumber::one() ;
    }

    /// Increment the nonce of an account. This helps us keep track of how many transactions
    /// each account has made.
    pub fn inc_nonce(&mut self, who: &T::AccountId) {
        let nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero()) ;
        let new_nonce = nonce + T::Nonce::one()  ;
        self.nonce.insert(who.clone(), new_nonce) ;
    }
}

#[cfg(test)]
mod test {
    #[test] 
    fn init_system() {
        struct TestConfig ;
        impl crate::system::Config for TestConfig {
            type AccountId = String ;
            type BlockNumber = u32 ;
            type Nonce = u32 ;
        } 

        // Instantiating a system struct.
        let mut system = crate::system::Pallet::<TestConfig>::new() ;

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