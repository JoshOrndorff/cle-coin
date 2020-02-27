/// A Pallet that controls two basic parameters of PoW blockchains through
/// on-chain governance.
///
/// The parameters controlled are:
/// * Difficulty
/// * BlockReward

use frame_support::{decl_module, decl_storage, decl_event, dispatch::DispatchResult, weights::SimpleDispatchInfo};
use system::{ensure_root, ensure_none};
use sp_core::U256;
use frame_support::{ensure, traits::Currency};
use codec::{Encode, Decode};
use sp_inherents::InherentIdentifier;
use sp_runtime::RuntimeString;
use sp_inherents::{IsFatalError, InherentData};
use sp_std::vec::Vec;
use sp_inherents::ProvideInherent;
#[cfg(feature = "std")]
use sp_inherents::ProvideInherentData;


pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
	type RewardCurrency: Currency<Self::AccountId>;
}

type BalanceOf<T> = <<T as Trait>::RewardCurrency as Currency<<T as system::Trait>::AccountId>>::Balance;

decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
		Difficulty get(fn difficulty) config(): U256 = 1000.into();
		Reward get(fn reward) config(): BalanceOf<T> = 1.into();
		Author: Option<T::AccountId>;
	}
}

decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		/// Root call to update difficulty
		pub fn set_difficulty(origin, new_difficulty: U256) -> DispatchResult {
			ensure_root(origin)?;

			Difficulty::put(new_difficulty);

			Self::deposit_event(RawEvent::DifficultySet(new_difficulty));
			Ok(())
		}

		/// Root call to update reward amount
		pub fn set_reward(origin, new_reward: BalanceOf<T>) -> DispatchResult {
			ensure_root(origin)?;

			Reward::<T>::put(new_reward);

			Self::deposit_event(RawEvent::RewardSet(new_reward));
			Ok(())
		}

		/// Inherent for the author to claim their reward
		#[weight = SimpleDispatchInfo::FixedOperational(10_000)]
		fn set_author(origin, author: T::AccountId) {
			ensure_none(origin)?;
			ensure!(<Self as Store>::Author::get().is_some(), "Author Already Set");

			<Self as Store>::Author::put(author);
		}
		/// When ending the block, actually pay the reward
		fn on_finalize() {
			if let Some(author) = <Self as Store>::Author::get() {
				drop(T::RewardCurrency::deposit_creating(&author, Reward::<T>::get()));
			}

			<Self as Store>::Author::kill();
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

pub const INHERENT_IDENTIFIER: InherentIdentifier = *b"rewards_";

#[derive(Encode)]
#[cfg_attr(feature = "std", derive(Debug, Decode))]
pub enum InherentError {
	Other(RuntimeString),
}

impl IsFatalError for InherentError {
	fn is_fatal_error(&self) -> bool {
		match *self {
			InherentError::Other(_) => true,
		}
	}
}

impl InherentError {
	/// Try to create an instance ouf of the given identifier and data.
	#[cfg(feature = "std")]
	pub fn try_from(id: &InherentIdentifier, data: &[u8]) -> Option<Self> {
		if id == &INHERENT_IDENTIFIER {
			<InherentError as codec::Decode>::decode(&mut &data[..]).ok()
		} else {
			None
		}
	}
}

pub type InherentType = Vec<u8>;

#[cfg(feature = "std")]
pub struct InherentDataProvider(pub InherentType);

#[cfg(feature = "std")]
impl ProvideInherentData for InherentDataProvider {
	fn inherent_identifier(&self) -> &'static InherentIdentifier {
		&INHERENT_IDENTIFIER
	}

	fn provide_inherent_data(&self, inherent_data: &mut InherentData) -> Result<(), sp_inherents::Error> {
		inherent_data.put_data(INHERENT_IDENTIFIER, &self.0)
	}

	fn error_to_string(&self, error: &[u8]) -> Option<String> {
		InherentError::try_from(&INHERENT_IDENTIFIER, error).map(|e| format!("{:?}", e))
	}
}

impl<T: Trait> ProvideInherent for Module<T> {
	type Call = Call<T>;
	type Error = InherentError;
	const INHERENT_IDENTIFIER: InherentIdentifier = INHERENT_IDENTIFIER;

	fn create_inherent(data: &InherentData) -> Option<Self::Call> {
		let author_raw = data.get_data::<InherentType>(&INHERENT_IDENTIFIER)
			.expect("Gets and decodes anyupgrade inherent data")?;

		let author = T::AccountId::decode(&mut &author_raw[..])
			.expect("Decodes author raw inherent data");

		Some(Call::set_author(author))
	}

	fn check_inherent(_call: &Self::Call, _data: &InherentData) -> Result<(), Self::Error> {
		Ok(())
	}
}
