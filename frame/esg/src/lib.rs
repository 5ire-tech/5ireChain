#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;
#[cfg(test)]
pub mod tests;

pub mod weights;

pub mod traits;

pub trait Sustainability<AccountId> {
	fn get_score_of(company: firechain_runtime_core_primitives::opaque::AccountId) -> u16;
}

#[frame_support::pallet]
pub mod pallet {
	use crate::{traits::ERScoresTrait, weights::WeightInfo};
	use core::num::IntErrorKind;
	use frame_support::{
		pallet_prelude::{DispatchResult, *},
		WeakBoundedVec,
	};
	use frame_system::pallet_prelude::*;
	use serde_json::Value;
	use sp_std::vec::Vec;
	use sp_core::H160;
	use firechain_runtime_core_primitives::opaque::AccountId as AccIdEth;
	use fp_account::AccountId20;
	

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	const MAX_ESG_SCORE: u16 = 100;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		#[pallet::constant]
		type MaxFileSize: Get<u32>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;

	}

	#[pallet::storage]
	#[pallet::getter(fn get_score_of)]
	pub type ESGScoresMap<T> =
		StorageMap<_, Blake2_128Concat, AccIdEth, u16, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_oracle_sudo)]
	pub type SudoOraclesStore<T> =
		StorageValue<_, Vec<AccIdEth>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_oracle_nsudo)]
	pub type NonSudoOraclesStore<T> =
		StorageValue<_, Vec<AccIdEth>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ESGStored { caller: AccIdEth },

		NewOracleRegistered { is_sudo: bool, oracle: AccIdEth },

		OracleDeRegistered { is_sudo: bool, oracle: AccIdEth },

		ESGStoredWithSkip { caller: AccIdEth, skipped_indeces: Vec<u16> },
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
		/// checks if given input `ip` vec of bytes represents a invalid account id
		/// returns true for invalid one
		/// false for valid one
		fn not_valid_acc_id_format(ip: &[u8]) -> bool {
			ip.len() != 42 ||
			!ip.starts_with(b"0x") ||
			!ip[2..].iter().all(|&c| c.is_ascii_hexdigit())
		}

		/// tries to resolve origin into account id
		/// depending on txn being simple or sudo signed, in which case returns Some()
		/// returns None if unsigned otherwise
		fn try_resolve(origin: &OriginFor<T>, oracle: &AccIdEth) -> (Option<AccIdEth>, bool) {
			match origin.clone().into() {
				Ok(frame_system::RawOrigin::Root) => (Some(oracle.clone()), true),
				Ok(frame_system::RawOrigin::Signed(id)) => (
					Some(AccountId20::from(H160::from_slice(&&id.encode().as_slice()[0..20]))), 
					false
				),
				_ => (None, false),
			}
		}

		/// checks if given id is an oracle of any kind
		fn is_an_oracle(acc_id: &AccIdEth) -> bool {
			<SudoOraclesStore<T>>::get().contains(acc_id) ||
				<NonSudoOraclesStore<T>>::get().contains(acc_id)
		}

		/// checks given id is of a sudo oracle
		fn is_sudo_oracle(oracle: &AccIdEth) -> bool {
			<SudoOraclesStore<T>>::get().contains(oracle)
		}

		/// stores an id as an oracle into respective storage kind as per `is_sudo`
		fn store_oracle(oracle: &AccIdEth, is_sudo: bool) {
			let fn_mutate = |oracles: &mut Vec<AccIdEth>| {
				oracles.push(oracle.clone())
			};

			match is_sudo {
				true => <SudoOraclesStore<T>>::mutate(fn_mutate),
				false => <NonSudoOraclesStore<T>>::mutate(fn_mutate),
			}
		}

		/// discard an oracle from respective storage kind as per `is_sudo`
		fn un_store_oracle(oracle: &AccIdEth, is_sudo: bool) -> DispatchResult {
			let fn_mutate = |oracles: &mut Vec<AccIdEth>| {
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

		/// this function is determined to return a u16
		/// following treated as 0 (default):
		/// - alphanumeric value
		/// - negative value
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

		fn hexstr2bytes_unsafe(s: &str) -> Vec<u8> {
			s.as_bytes()
			.chunks(2)
			.map(|chunk| u8::from_str_radix(core::str::from_utf8(chunk).unwrap(), 16).unwrap())
			.collect()
		}
		/// tries to parse accoundId from given json value under key 'account'
		/// returns Some(AccountId) if everything is ok or
		/// returns None if something, like invalid string provided, about it is not ok
		pub fn try_parse_acc_id(acc_val: Option<&serde_json::Value>) -> Option<AccIdEth> {
			let acc = acc_val.unwrap_or(&Value::Null).as_str().unwrap_or("");

			if Self::not_valid_acc_id_format(acc.as_bytes()) {
				return None
			}
			Some(AccountId20::from(H160::from_slice(Self::hexstr2bytes_unsafe(&acc[2..]).as_slice())))
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
			let signer = AccountId20::from(H160::from_slice(&&signer.encode().as_slice()[0..20]));

			// return an error if caller is not an oracle
			if !Self::is_an_oracle(&signer) {
				return Err(Error::<T>::CallerNotAnOracle.into())
			}

			// try decode bytes to utf8 string slice
			let converted_string = core::str::from_utf8(&json_str_bytes)
				.map_or_else(|_| Err(Error::<T>::InvalidUTF8), Ok)?;

			// try deserialize utf8 string slice
			let esg_info: Value = serde_json::from_str(converted_string)
				.map_or_else(|_| Err(Error::<T>::InvalidJson), Ok)?;

			let esg_data = esg_info.as_array().map_or_else(|| Err(Error::<T>::InvalidJson), Ok)?;

			// to keep track of indexes of invalid account ids in given data
			let mut skipped_indeces = Vec::<u16>::new();

			esg_data.iter().enumerate().for_each(|(i, ed)| {
				match Self::try_parse_acc_id(ed.get("account")) {
					Some(id) =>
						<ESGScoresMap<T>>::mutate(&id, |v| *v = Self::parse_score(ed.get("score"))),
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
			oracle: AccIdEth,
			is_sudo_oracle: bool,
		) -> DispatchResult {
			let (id, is_root) = Self::try_resolve(&origin, &oracle);

			let acc_id = match id {
				Some(id) => id,
				// if not signed
				None => return Err(Error::<T>::NotSigned.into()),
			};

			log::info!("#@! root: {is_root} same?: {}", acc_id.eq(&oracle));

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
			oracle: AccIdEth,
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

	impl<T: Config> ERScoresTrait<AccIdEth> for Pallet<T> {
		fn get_score_of(org: AccIdEth) -> u16 {
			ESGScoresMap::<T>::get(&org)
		}
		fn chilled_validator_status(_org: AccIdEth) {}
		fn reset_chilled_validator_status(_org: AccIdEth) {}
		fn reset_score_after_era_for_chilled_active_validator() {}
		fn reset_score_of_chilled_waiting_validator(_org: AccIdEth) {}
	}
}
