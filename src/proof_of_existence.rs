use core::fmt::Debug ;
use std::collections::BTreeMap ;
use crate::support::DispatchResult ;

/// The Config trait for our Proof of Existence pallet.
/// It contains the types AccountId & Content of a user.
pub trait Config: crate::system::Config {
    /// A type representing the content that can be claimed using this pallet.
    /// The content could be bytes or hash of that content. It's upto the Runtime developer.
    type Content: Debug + Ord ;
}

/// This is the Proof of Existence pallet.
/// It is a simple pallet that allows accounts to claim existence of some data.
#[derive(Debug)]
pub struct Pallet<T: Config> {
    /// A simple storage map from content to the owner of that content.
    /// Accounts can make multiple claims, but a claim can only be owned by a particular owner.
    claims: BTreeMap<T::Content, T::AccountId> 
}

impl<T:Config> Pallet<T> {
    /// Create a new instance of out POE pallet.
    pub fn new() -> Self {
        Self {
            claims: BTreeMap::new()
        }
    }

    /// Get the owner(if any) of a claim.
    pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
        self.claims.get(claim)
    }
}

// Only these function will be called by the user from this pallet, so we will separate these from the other 
// pallet functions and only add rust macro to this implementation of our Pallet.
#[macros::call]
impl<T: Config> Pallet<T> {
    /// Create a claim on behalf of the 'caller'.
    /// If the content is already claimed by some other user, the function will return an error.
    pub fn create_claim(
        &mut self, 
        caller: T::AccountId, 
        claim: T::Content
    ) -> DispatchResult {
        if self.claims.contains_key(&claim) {
            return Err("This content is already been claimed.");
        }
        self.claims.insert(claim, caller) ;
        Ok(())
    }

    /// Revoke an existing claim on some content.
    /// This function should only succeed if the caller is owner of an existing claim.
    /// This function will result into an error if the claim does not exist, or if the caller is not the owner of the claim.
    pub fn revoke_claim(
        &mut self,
        caller: T::AccountId,
        claim: T::Content
    ) -> DispatchResult {
        // Get the owner of the claim to be revoked.
        let owner = self.get_claim(&claim).ok_or("Claim does not exist.") ?;

        // Check whether the caller is the owner of the claim.
        if *owner != caller{
            return Err("This content is owned by some other user.");
        }

        // Remove the claim if above check passes.
        self.claims.remove(&claim) ;
        Ok(())
    }
}


// Since we are using rust macros, the enum 'Call' and implementation of 'Dispatch' will be provided by 
// rust macros themselves.

// /// A public enum which describes the calls we want to expose to the dispatcher.
// // We should expect that the caller of each call will be provided by the dispatcher, and not included as a 
// // parameter of the call.
// pub enum Call<T: Config> {
//     CreateClaim { claim: T::Content },
//     RevokeClaim {claim: T::Content },
// }

// /// Implementation of the dispatch logic, mapping from 'POECall' to the appropriate underlying functions we want to execute.
// impl<T: Config> crate::support::Dispatch for Pallet<T> {
//     type Caller = T::AccountId ;
//     type Call = Call<T>;

//     fn dispatch(&mut self,
//         caller: Self::Caller,
//         call: Self::Call
//     ) -> DispatchResult {
//         match call {
//             Call::CreateClaim { claim } => {
//                 self.create_claim(caller, claim) ?; 
//             },
//             Call::RevokeClaim { claim } => {
//                 self.revoke_claim(caller, claim) ?;
//             },
//         } 
//         Ok(())
//     }
// }

#[cfg(test)]
mod test {
    struct TestConfig ;
    impl crate::proof_of_existence::Config for TestConfig {
        type Content = &'static str ;
    }

    impl crate::system::Config for TestConfig {
        type AccountId = String ;
        type BlockNumber = u32 ;
        type Nonce = u32 ;
    }

    #[test]
    fn init_proof_of_existence() {
        let mut proof_of_existence = crate::proof_of_existence::Pallet::<TestConfig>::new() ;
        
        let alice = "alice".to_string() ;
        let bob = "bob".to_string() ;

        // In initial state, "hello" is not claimed by anyone.
        assert_eq!(proof_of_existence.get_claim(&"hello"), None) ;
        
        // Creating claim for 'alice'.
        let _ = proof_of_existence.create_claim(alice.clone(), "hello");
        assert_eq!(proof_of_existence.get_claim(&"hello"), Some(&alice)) ;

        // Since alice is owner of claim, "hello", bob cannot claim this content.
        assert_eq!(
            proof_of_existence.create_claim(bob.clone(), "hello"),
            Err("This content is already been claimed.")
        ) ;

        // Since alice is owner of claim, "hello", bob cannot revoke this claim.
        assert_eq!(
            proof_of_existence.revoke_claim(bob.clone(), "hello"),
            Err("This content is owned by some other user.")
        ) ;
        
        // Revoke claim "hello" for alice.
        let _ = proof_of_existence.revoke_claim(alice, "hello") ;
        
        // Now, bob can claim "hello".
        let _ = proof_of_existence.create_claim(bob.clone(), "hello");
        assert_eq!(proof_of_existence.get_claim(&"hello"), Some(&bob)) ;
    }
}