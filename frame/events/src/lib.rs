#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
pub use pallet::*;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
use scale_info::prelude::string::String;
pub use weights::*;
// pub mod rpc;
pub mod types;

#[frame_support::pallet]
pub mod pallet {

	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use types::*;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;
	}

	#[pallet::storage]
	#[pallet::getter(fn _event_ids)]
	pub type _event_ids<T> = StorageValue<_, u32>;

	/// Get the details of a events by its' id.
	#[pallet::storage]
	#[pallet::getter(fn event_by_id)]
	pub type EventById<T: Config> = StorageMap<_, Twox64Concat, u32, EventAuction>;

	#[pallet::storage]
	#[pallet::getter(fn _token_ids)]
	pub type _token_ids<T> = StorageValue<_, u32>;

	/// Get the details of a tokens by its' id.
	#[pallet::storage]
	#[pallet::getter(fn token_by_id)]
	pub type TokenById<T: Config> = StorageMap<_, Twox64Concat, u32, TokenAuction>;

	#[pallet::storage]
	#[pallet::getter(fn _donations_ids)]
	pub type _donations_ids<T> = StorageValue<_, u32>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SomethingStored { something: u32, who: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::create_event())]
		pub fn create_event(
			origin: OriginFor<T>,
			_event_uri: String,
			_dao_id: String,
			_user_id: String,
			_feed: String,
		) -> DispatchResult {
			let mut new_id = 0;
			match <_event_ids<T>>::try_get() {
				Ok(old) => {
					new_id = old;
					<_event_ids<T>>::put(new_id + 1);
				},
				Err(_) => {
					<_event_ids<T>>::put(1);
				},
			}

			let new_event = &mut EventAuction {
				id: new_id,
				dao_id:_dao_id,
				user_id:_user_id,
				event_uri:_event_uri.clone(),
				event_wallet:_event_uri.clone(),
				raised: String::from("0"),
				status:String::from("0"),
			};

			EventById::<T>::insert(new_id, new_event);
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::claim_token())]
		pub fn claim_token(
			origin: OriginFor<T>,
			_event_id: u32,
			_token_uri: String,
			_token_wallet: String,
		) -> DispatchResult {
			let mut new_id = 0;
			match <_token_ids<T>>::try_get() {
				Ok(old) => {
					new_id = old;
					<_token_ids<T>>::put(new_id + 1);
				},
				Err(_) => {
					<_token_ids<T>>::put(1);
				},
			}

			let new_token = &mut TokenAuction {
				id: new_id,
				event_id: _event_id,
				token_uri: _token_uri.clone(),
				token_wallet: _token_wallet.clone(),
			};

			TokenById::<T>::insert(new_id, new_token);
			Ok(())
		}
	}
}
