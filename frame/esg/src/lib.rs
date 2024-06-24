#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]

pub mod tests;

pub mod traits;

pub mod weights;

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;

pub trait Sustainability<AccountId> {
	fn get_score_of(company: AccountId) -> u16;
}

#[frame_support::pallet]
pub mod pallet {
	use fp_account::AccountId20;
use frame_support::{
		WeakBoundedVec,
		pallet_prelude::{DispatchResult, *},
	};
	use sp_core::H160;
	use sp_std::vec::Vec;
	use serde_json::Value;
	use core::str::FromStr;
	use core::num::IntErrorKind;
	use frame_system::pallet_prelude::*;
	use crate::{traits::ERScoresTrait, weights::WeightInfo};
	
	const MAX_ESG_SCORE: u16 = 100;
	const ACC_KEY: &str = "account";
	const SCORE_KEY: &str = "score";

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		#[pallet::constant]
		type MaxFileSize: Get<u32>;		
		type WeightInfo: WeightInfo;
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	
	#[pallet::storage]
	#[pallet::getter(fn get_oracle_sudo)]
	pub type SudoOraclesStore<T> =
	StorageValue<_, Vec<<T as frame_system::Config>::AccountId>, ValueQuery>;
	
	#[pallet::storage]
	#[pallet::getter(fn get_oracle_nsudo)]
	pub type NonSudoOraclesStore<T> =
	StorageValue<_, Vec<<T as frame_system::Config>::AccountId>, ValueQuery>;
	
	#[pallet::storage]
	#[pallet::getter(fn get_score_of)]
	pub type ESGScoresMap<T> =
		StorageMap<_, Blake2_128Concat, <T as frame_system::Config>::AccountId, u16, ValueQuery>;
	
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ESGStored { caller: <T as frame_system::Config>::AccountId },

		OracleDeRegistered { is_sudo: bool, oracle: <T as frame_system::Config>::AccountId },
		
		NewOracleRegistered { is_sudo: bool, oracle: <T as frame_system::Config>::AccountId },

		ESGStoredWithSkip { caller: <T as frame_system::Config>::AccountId, skipped_indeces: Vec<u16> },
	}

	#[pallet::error]
	pub enum Error<T> {
		NotSigned,
		InvalidUTF8,
		InvalidJson,
		OracleNotExist,
		CallerNotAnOracle,
		OracleRegisteredAlready,
		CallerNotRootOrSudoOracle,
	}

	impl<T: Config> Pallet<T> {
		
		fn not_valid_addr(ip: &[u8]) -> bool {
			ip.len() != 42 ||
			!ip.starts_with(b"0x") ||
			!ip[2..].iter().all(|&c| c.is_ascii_hexdigit())
		}

		fn try_resolve(origin: &OriginFor<T>, oracle: &<T as frame_system::Config>::AccountId) -> (Option<<T as frame_system::Config>::AccountId>, bool) {
			match origin.clone().into() {
				Ok(frame_system::RawOrigin::Root) => (Some(oracle.clone()), true),
				Ok(frame_system::RawOrigin::Signed(id)) => (Some(id), false),
				_ => (None, false),
			}
		}

		fn is_an_oracle(acc_id: &<T as frame_system::Config>::AccountId) -> bool {
			<SudoOraclesStore<T>>::get().contains(acc_id) ||
				<NonSudoOraclesStore<T>>::get().contains(acc_id)
		}

		fn is_sudo_oracle(oracle: &<T as frame_system::Config>::AccountId) -> bool {
			<SudoOraclesStore<T>>::get().contains(oracle)
		}

		fn store_oracle(oracle: &<T as frame_system::Config>::AccountId, is_sudo: bool) {
			let fn_mutate = |oracles: &mut Vec<<T as frame_system::Config>::AccountId>| {
				oracles.push(oracle.clone())
			};

			match is_sudo {
				true => <SudoOraclesStore<T>>::mutate(fn_mutate),
				false => <NonSudoOraclesStore<T>>::mutate(fn_mutate),
			}
		}

		fn un_store_oracle(oracle: &<T as frame_system::Config>::AccountId, is_sudo: bool) -> DispatchResult {
			let fn_mutate = |oracles: &mut Vec<<T as frame_system::Config>::AccountId>| {
				for (i, orc) in oracles.iter().enumerate() {
					if orc.eq(oracle) {
						oracles.remove(i);
						return Ok(())
					}
				}
				Err(Error::<T>::OracleNotExist.into())
			};

			match is_sudo {
				true => <SudoOraclesStore<T>>::mutate(fn_mutate),
				false => <NonSudoOraclesStore<T>>::mutate(fn_mutate),
			}
		}

		fn parse_score(score_val: Option<&serde_json::Value>) -> u16 {
			score_val
				.unwrap_or(&Value::Null)
				.as_str()
				.map_or(0u16, |s| {
					s.parse::<u16>().unwrap_or_else(|e| {
						if e.kind().eq(&IntErrorKind::PosOverflow) {
							MAX_ESG_SCORE
						} else {
							0u16
						}
					})
				})
				.clamp(0, MAX_ESG_SCORE)
		}

		pub fn try_parse_addr(acc_val: Option<&serde_json::Value>) -> Option<<T as frame_system::Config>::AccountId> {
			let acc = acc_val.unwrap_or(&Value::Null).as_str().unwrap_or("");

			if Self::not_valid_addr(acc.as_bytes()) {
				return None
			}

			H160::from_str(&acc[2..])
			.map(Into::into)
			.ok()
			.and_then(|acc_id: AccountId20| {
				T::AccountId::decode(&mut acc_id.as_ref()).ok()
			})
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::upsert_esg_scores())]
		pub fn upsert_esg_scores(
			origin: OriginFor<T>,
			json_str_bytes: WeakBoundedVec<u8, T::MaxFileSize>,
		) -> DispatchResult {
			let signer = ensure_signed(origin)?;

			if !Self::is_an_oracle(&signer) {
				return Err(Error::<T>::CallerNotAnOracle.into())
			}

			let converted_string = core::str::from_utf8(&json_str_bytes)
				.map_or_else(|_| Err(Error::<T>::InvalidUTF8), Ok)?;

			let esg_info: Value = serde_json::from_str(converted_string)
				.map_or_else(|_| Err(Error::<T>::InvalidJson), Ok)?;

			let esg_data = esg_info.as_array().map_or_else(|| Err(Error::<T>::InvalidJson), Ok)?;

			let mut skipped_indeces = Vec::<u16>::new();

			esg_data.iter().enumerate().for_each(|(i, ed)| {
				match Self::try_parse_addr(ed.get(ACC_KEY)) {
					Some(id) =>
						<ESGScoresMap<T>>::mutate(&id, |v| *v = Self::parse_score(ed.get(SCORE_KEY))),
					// acc_id is either invalid or
					// not found in json data under current index
					None => skipped_indeces.push(i as u16),
				};
			});

			if !skipped_indeces.is_empty() {
				Self::deposit_event(Event::ESGStoredWithSkip {
					skipped_indeces,
					caller: signer.clone(),
				});
				return Ok(())
			}
			Self::deposit_event(Event::ESGStored { caller: signer.clone() });
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::register_an_oracle())]
		pub fn register_an_oracle(
			origin: OriginFor<T>,
			oracle: <T as frame_system::Config>::AccountId,
			is_sudo_oracle: bool,
		) -> DispatchResult {
			let (id, is_root) = Self::try_resolve(&origin, &oracle);

			let acc_id = match id {
				Some(id) => id,
				// if not signed
				None => return Err(Error::<T>::NotSigned.into()),
			};

			if Self::is_an_oracle(&oracle) {
				return Err(Error::<T>::OracleRegisteredAlready.into())
			}

			if is_root || Self::is_sudo_oracle(&acc_id) {
				Self::store_oracle(&oracle, is_sudo_oracle);
			} else {
				return Err(Error::<T>::CallerNotRootOrSudoOracle.into())
			}
			Self::deposit_event(Event::NewOracleRegistered {
				oracle: oracle.clone(),
				is_sudo: is_sudo_oracle,
			});
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::deregister_an_oracle())]
		pub fn deregister_an_oracle(
			origin: OriginFor<T>,
			oracle: <T as frame_system::Config>::AccountId,
			is_sudo_oracle: bool,
		) -> DispatchResult {
			let o = ensure_root(origin);

			if o.is_err() {
				return Err(DispatchError::BadOrigin)
			}

			if !Self::is_an_oracle(&oracle) {
				return Err(Error::<T>::OracleNotExist.into())
			}

			let un_stored = Self::un_store_oracle(&oracle, is_sudo_oracle);
			if un_stored.is_ok() {
				Self::deposit_event(Event::OracleDeRegistered {
					oracle: oracle.clone(),
					is_sudo: is_sudo_oracle,
				});
				return Ok(())
			}
			return un_stored
		}
	}

	impl<T: Config> ERScoresTrait<<T as frame_system::Config>::AccountId> for Pallet<T> {
		fn get_score_of(org: <T as frame_system::Config>::AccountId) -> u16 {
			ESGScoresMap::<T>::get(&org)
		}
		fn chilled_validator_status(_org: <T as frame_system::Config>::AccountId) {}
		fn reset_chilled_validator_status(_org: <T as frame_system::Config>::AccountId) {}
		fn reset_score_after_era_for_chilled_active_validator() {}
		fn reset_score_of_chilled_waiting_validator(_org: <T as frame_system::Config>::AccountId) {}
	}
}
