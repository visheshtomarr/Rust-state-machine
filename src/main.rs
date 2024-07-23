mod balances ;
mod system ;

/// These are the concrete types we will be using in our simple state machine.
/// Modules are configured for these types directly, and they satisfy all of our trait requirements.
mod types {
	pub type AccountId = String ;
	pub type Balance = u128 ; 
	pub type BlockNumber = u32 ;
	pub type Nonce = u32 ;
}

/// This is our main Runtime.
/// It accumulates all the different pallets we want to use.
#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<types::AccountId, types::BlockNumber, types::Nonce>,
	balances: balances::Pallet<types::AccountId, types::Balance>,
}

impl Runtime {
	/// Create a new instance of our main Runtime, by creating a new instance of each pallet.
	pub fn new() -> Self {
		Self {
			system: system::Pallet::new(),
			balances: balances::Pallet::new(),
		}
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
