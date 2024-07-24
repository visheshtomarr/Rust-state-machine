/// The most primitive representation of a Blockchain block.
pub struct Block<Header, Extrinsic> {
    /// The block header contains the metadata about the block.
    pub header: Header,
    /// The extrinsics represents the state transitions to be executed in this block.
    pub extrinsics: Vec<Extrinsic>,
}

/// We are using an extremely simplified header which only contains the current block number.
/// A real blockchain like Polkadot will also have the following :
/// - parent hash
/// - state root
/// - extrinsic root
/// - consensus digest
/// - etc..
pub struct Header<BlockNumber> {
    pub block_number: BlockNumber,
} 

/// This is an "extrinsic", which is an external message from outside of the blockchain.
/// This simplified version of extrinsic tells us who is making the "Call" and which call they are making.
pub struct Extrinsic<Caller, Call> {
    pub caller: Caller,
    pub call: Call, 
}

/// The "Result" type for our Runtime. When everything completes successfully, we return an "Ok(())", else
/// we return a static error message.
pub type DispatchResult = Result<(), &'static str> ;

/// A trait which allows us to dispatch an incoming extrinsic to the appropriate state transition function call.
pub trait Dispatch {
    /// The type to identify the caller of the function.
    type Caller ;
    /// The state transition function call the caller is trying to access.
    type Call ;
    /// This function takes up a 'caller' and the 'call' he/she is trying to make, and returns a 'Result'
    /// based on the outcome of that call.
    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult ;
}