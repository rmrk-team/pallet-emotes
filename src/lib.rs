#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{DispatchError, RuntimeDebug};

#[cfg_attr(feature = "std", derive(PartialEq, Eq))]
#[derive(Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct Emoticons {
    pub byte: String,
    pub unicode: String,
}

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types it depends on.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::storage]
    #[pallet::getter(fn emote_entity)]
    /// Stores emoted entities
    pub type EmoteEntity<T: Config> = StorageDoubleMap<_, Twox64Concat, u32, Twox64Concat, T::AccountId, Vec<Emoticons>>;

    // Errors.
    #[pallet::error]
    pub enum Error<T> {
        EmoteUsed,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
    } 

    // TODO: Our pallet's genesis configuration.

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(100)]
        pub fn emote(
            origin: OriginFor<T>,
            unicode: String,
            entity_id: u32
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let mut existing_emote = <EmoteEntity<T>>::get(&entity_id, &who).unwrap_or_else(Vec::new);
            ensure!(!existing_emote.iter().any(|row| row.unicode == unicode), Error::<T>::EmoteUsed);

            Ok(())
        }
    }

    // helper functions
    impl<T: Config> Pallet<T> {
    }
}
