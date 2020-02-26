/// A Pallet that controls two basic parameters of PoW blockchains through
/// on-chain governance.
///
/// The parameters controlled are:
/// * Difficulty
/// * BlockReward

use frame_support::{decl_module, decl_storage, decl_event, dispatch::DispatchResult};
use system::ensure_root;
use sp_core::U256;
use frame_support::traits::Currency;

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
	type RewardCurrency: Currency<Self::AccountId>;
}

type BalanceOf<T> = <<T as Trait>::RewardCurrency as Currency<<T as system::Trait>::AccountId>>::Balance;

decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
		Difficulty get(fn difficulty) config(): U256 = 200.into();
		Reward get(fn reward) config(): BalanceOf<T> = 1.into();
	}
}

decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		pub fn set_difficulty(origin, new_difficulty: U256) -> DispatchResult {
			ensure_root(origin)?;

			Difficulty::put(new_difficulty);

			Self::deposit_event(RawEvent::DifficultySet(new_difficulty));
			Ok(())
		}

		pub fn set_reward(origin, new_reward: BalanceOf<T>) -> DispatchResult {
			ensure_root(origin)?;

			Reward::<T>::put(new_reward);

			Self::deposit_event(RawEvent::RewardSet(new_reward));
			Ok(())
		}
	}
}

decl_event!(
	pub enum Event<T> where Balance = BalanceOf<T> {
		/// The PoW Difficulty has been set
		DifficultySet(U256),
		/// The Block Reward has bee nset
		RewardSet(Balance),
	}
);

/// tests for this module
#[cfg(test)]
mod tests {
	use super::*;

	use sp_core::H256;
	use frame_support::{impl_outer_origin, assert_ok, parameter_types, weights::Weight};
	use sp_runtime::{
		traits::{BlakeTwo256, IdentityLookup}, testing::Header, Perbill,
	};

	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	// For testing the module, we construct most of a mock runtime. This means
	// first constructing a configuration type (`Test`) which `impl`s each of the
	// configuration traits of modules we want to use.
	#[derive(Clone, Eq, PartialEq)]
	pub struct Test;
	parameter_types! {
		pub const BlockHashCount: u64 = 250;
		pub const MaximumBlockWeight: Weight = 1024;
		pub const MaximumBlockLength: u32 = 2 * 1024;
		pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
	}
	impl system::Trait for Test {
		type Origin = Origin;
		type Call = ();
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type Event = ();
		type BlockHashCount = BlockHashCount;
		type MaximumBlockWeight = MaximumBlockWeight;
		type MaximumBlockLength = MaximumBlockLength;
		type AvailableBlockRatio = AvailableBlockRatio;
		type Version = ();
		type ModuleToIndex = ();
	}
	impl Trait for Test {
		type Event = ();
	}
	type TemplateModule = Module<Test>;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> sp_io::TestExternalities {
		system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
	}

	#[test]
	fn it_works_for_default_value() {
		new_test_ext().execute_with(|| {
			// Just a dummy test for the dummy funtion `do_something`
			// calling the `do_something` function with a value 42
			assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
			// asserting that the stored value is equal to what we stored
			assert_eq!(TemplateModule::something(), Some(42));
		});
	}
}
