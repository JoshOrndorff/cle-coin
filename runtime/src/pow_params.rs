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
		Difficulty get(fn difficulty) config(): U256 = 5000.into();
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
