#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    use sp_std::vec::Vec;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        /// Length check for claim
        type MaxClaimLengthLimitation: Get<u32>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // The pallet's runtime storage items.
    // https://substrate.dev/docs/en/knowledgebase/runtime/storage
    #[pallet::storage]
    #[pallet::getter(fn proofs)]
    // Learn more about declaring storage items:
    // https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
    pub type Proofs<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber), ValueQuery>;

    // Pallets use events to inform users when important changes are made.
    // https://substrate.dev/docs/en/knowledgebase/runtime/events
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event on a claim is created
        /// parameters. [claim, who]
        ClaimCreated(Vec<u8>, T::AccountId),
        /// Event on a claim is revoked
        /// parameters. [claim, who]
        ClaimRevoked(Vec<u8>, T::AccountId),
        /// Event on a claim is transfered from A to B
        /// parameters. [claim, from, to]
        ClaimTransfered(Vec<u8>, T::AccountId, T::AccountId),
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        ProofAlreadyClaimedError,
        ProofNotFoundError,
        ProofOwnershipError,
        ProofSizeExceededError,
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn create_claim(origin: OriginFor<T>, proof: Vec<u8>) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            ensure!(
                T::MaxClaimLengthLimitation::get() >= proof.len() as u32,
                Error::<T>::ProofSizeExceededError
            );
            ensure!(
                !Proofs::<T>::contains_key(&proof),
                Error::<T>::ProofAlreadyClaimedError
            );
            let current_block = <frame_system::Module<T>>::block_number();
            Proofs::<T>::insert(&proof, (&who, current_block));
            Self::deposit_event(Event::ClaimCreated(proof, who));
            Ok(().into())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn revoke_claim(origin: OriginFor<T>, proof: Vec<u8>) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::ProofNotFoundError);
            let (owner, _) = Proofs::<T>::get(&proof);
            ensure!(who == owner, Error::<T>::ProofOwnershipError);
            Proofs::<T>::remove(&proof);
            Self::deposit_event(Event::ClaimRevoked(proof, who));
            Ok(().into())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn transfer_claim(origin: OriginFor<T>, proof: Vec<u8>, to: T::AccountId) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::ProofNotFoundError);
            let (owner, block_number) = Proofs::<T>::get(&proof);
            ensure!(who == owner, Error::<T>::ProofOwnershipError);
            Proofs::<T>::insert(&proof, (to.clone(), block_number));
            Self::deposit_event(Event::ClaimTransfered(proof, who, to));
            Ok(().into())
        }
    }
}