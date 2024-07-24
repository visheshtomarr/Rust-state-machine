mod balances ;
mod system ;
mod support ;

use crate::support::Dispatch ;

/// These are the concrete types we will be using in our simple state machine.
/// Modules are configured for these types directly, and they satisfy all of our trait requirements.
mod types {
	pub type AccountId = String ;
	pub type Balance = u128 ; 
	pub type BlockNumber = u32 ;
	pub type Nonce = u32 ;
	pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall> ;
	pub type Header = crate::support::Header<BlockNumber> ;
	pub type Block = crate::support::Block<Header, Extrinsic> ;
}

/// These are the calls which are exposed to the outside world.
/// It is just an accumulation of the calls exposed by each pallets.
pub enum RuntimeCall {
}

/// This is our main Runtime.
/// It accumulates all the different pallets we want to use.
#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
}

impl system::Config for Runtime {
	type AccountId = types::AccountId ;
	type BlockNumber = types::BlockNumber ;
	type Nonce = types::Nonce ;
}

impl balances::Config for Runtime {
	type Balance = types::Balance ;
}

impl Runtime {
	/// Create a new instance of our main Runtime, by creating a new instance of each pallet.
	pub fn new() -> Self {
		Self {
			system: system::Pallet::new(),
			balances: balances::Pallet::new(),
		}
	}

	/// Execute a block of extrinsics. Incrememts the block number.
	pub fn execute_block(&mut self, block: types::Block) -> crate::support::DispatchResult {
		// Increment system's block number.
		self.system.inc_block_number() ;

		// Check the current block number 
		if self.system.block_number() != block.header.block_number {
			return Err("The current block number is invalid.") ;
		}

		for (i, crate::support::Extrinsic {caller, call}) in block.extrinsics.into_iter().enumerate() {
			// Increment the nonce of caller.
			self.system.inc_nonce(&caller) ;

			let _res = self.dispatch(caller, call).map_err(|e| {
				eprintln!(
					"Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
					block.header.block_number, i, e
				)
			}) ;
		}
		Ok(())
	} 
}

impl crate::support::Dispatch for Runtime {
	type Caller = types::AccountId ;
	type Call = RuntimeCall ;

	fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> support::DispatchResult {
		unimplemented!()
	} 
}

fn main() {
	// Instantiating a new instance of our Runtime.
	let mut runtime = Runtime::new() ;

	// Creating users
	let alice = "alice".to_string() ;
	let bob = "bob".to_string() ;
	let charlie = "charlie".to_string() ;

	// Set balance of "alice" to 100, allowing us to execute transactions.
	runtime.balances.set_balance(&alice, 100) ;

	// Start emulating a block.
	// Increment the block number in system.
	runtime.system.inc_block_number() ;

	// Assert the block number is what we expect.
	assert_eq!(runtime.system.block_number(), 1) ;

	// First transaction
	// Increment nonce of "alice".
	runtime.system.inc_nonce(&alice) ;

	// Transfer funds from "alice" to "bob". Handling possible error, in case "alice" doesn't have required funds.
	let _res = runtime
		.balances
		.transfer(alice.clone(), bob, 30)
		.map_err(|e| eprintln!("{}", e)) ;

	// Second transaction
	// Increment nonce of "alice" again.
	runtime.system.inc_nonce(&alice) ;

	// Transfer funds from "alice" to "charlie". Handling possible error, in case "alice" doesn't have required funds.
	let _res = runtime
	.balances
	.transfer(alice, charlie, 20)
	.map_err(|e| eprintln!("{}", e)) ;

	// Print our final runtime.
	println!("{:#?}", runtime) ;
}
