mod balances ;
mod system ;
mod support ;
mod proof_of_existence ;

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
	Balances(balances::Call<Runtime>),
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
	type Caller = <Runtime as system::Config>::AccountId ;
	type Call = RuntimeCall ;

	// Dispatch a call on behalf of the caller. Increments the caller's nonce.
	// This function allows us to identify which underlying module call we want to execute.
	fn dispatch(
		&mut self, 
		caller: Self::Caller,
		runtime_call: Self::Call
	) -> crate::support::DispatchResult {
		match runtime_call {
			RuntimeCall::Balances(call) => {
				self.balances.dispatch(caller, call) ?;
			},
		}
		Ok(())
	}
}

fn main() {
	// Instantiating a new instance of our Runtime.
	let mut runtime = Runtime::new() ;

	// Creating users.
	let alice = "alice".to_string() ;
	let bob = "bob".to_string() ;
	let charlie = "charlie".to_string() ;

	// Set balance of "alice" to 100, allowing us to execute transactions.
	runtime.balances.set_balance(&alice, 100) ;

	// Instantiating first block and executing extrinsics.
	let block_1 = types::Block{
		header: support::Header{
			block_number: 1,
		},
		extrinsics: vec![
			support::Extrinsic{
				caller: alice.clone(),
				call: RuntimeCall::Balances(balances::Call::Transfer { to: bob, amount: 30 })
			},
			support::Extrinsic{
				caller: alice,
				call: RuntimeCall::Balances(balances::Call::Transfer { to: charlie, amount: 20 })
			}],
	};

	// Executing block.
	runtime.execute_block(block_1).expect("Invalid block.") ; 

	// Print our final runtime.
	println!("{:#?}", runtime) ;
}
