use std::collections::BTreeMap ;

/// This is the system Pallet.
/// It handles low level state needed for our blockchain.
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
}