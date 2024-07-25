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