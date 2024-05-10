pallet Template Module


#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

#[cfg(test)]
mod mock;

#[#cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;


#[frame_support::pallet]
pub mod Pallet{

	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	
	#[pallet::pallet]
	pub struct Pallet<T>(_);
	
	#[pallet::config]
	pub trait Config:frame_system::Config{
	
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// A type representing the weights required by the dispatchables of this pallet.
		type WeightInfo: WeightInfo;
	
	}
	
	#[derive(Debug,Clone,PartialEq,Encode,Decode,Default,TypeInfo)]
	pub struct UserInfo{
		
		name:Vec<u8>,
		id : Vec<u8>,
		about_me:Vec<u8>
	
	}
	
	
	#[pallet::storage]
	#[pallet::getter(pub(super) info]
	pub type AccountToInfo<T:Config>=StorageMap<,Blate2_Concat,T::AccountId,UserInfo,OptionQuery>;
	
	
	#[pallet::event]
	#[pallet::generate_deposite(pub (super) deposite_event)]
	pub enum Event<T:Config>{
		
		UserCreated{User:T::AccountId}
		
	}
	
	
	

	
	
	
	#[pallet::error]
	
	
	#[pallet::call]
	impl<T:Config>Pallet<T>{
	
		#[pallet::weight(10_000+T::DbWeight::get().writes(1).ref_time())]
		
		pub fn register_user(origin:OriginFor<T>,username:Vec<u8>,userid:Vec<u8>,userabout:Vec<u8>)->DispatchResult{
		
		let sender=ensire_signed(origin)?;
		let user=UserInfo{username,userid,userabout};
		
		AccountToInfo::<T>::insert(&sender,user);
		Self::deposite_event(Event::<T>::UserCreated(&sender));
		Ok(());
		
			
		
		
		}
	
	}


}
