// Reward Pallet
// The Reward Pallet is a module in the Substrate blockchain framework designed to manage and distribute rewards to participants based on their contributions within the network.
// This pallet facilitates the allocation of rewards to validators and nominators for their involvement in staking activities.

#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::pallet_prelude::DispatchResult;
pub use pallet::*;
use pallet_staking::{ CurrentEra, Validators, ErasRewardPoints, ErasStakers, IndividualExposure };
use parity_scale_codec::Codec;
use scale_info::prelude::{ vec::Vec, fmt::Debug };
use sp_runtime::{ FixedPointOperand, traits::{ Convert, AtLeast32BitUnsigned } };
use frame_support::PalletId;
use pallet_staking::Rewards;
use frame_support::traits::{
	Currency,
	LockableCurrency,
	ValidatorSet,
	Get,
	//reward::Rewards,
	ExistenceRequirement,
	ExistenceRequirement::KeepAlive,
};
use frame_election_provider_support::{ ElectionDataProvider, DataProviderBounds };
use sp_runtime::traits::AccountIdConversion;
use sp_runtime::traits::Zero;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_staking::Config  {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type ValidatorSet: frame_support::traits::ValidatorSetWithIdentification<Self::AccountId>;
		type Validators: frame_support::traits::ValidatorSet<Self::AccountId>;
		type DataProvider: ElectionDataProvider<
			AccountId = <Self::ValidatorSet as ValidatorSet<Self::AccountId>>::ValidatorId,
			BlockNumber = BlockNumberFor<Self>
		>;
		#[pallet::constant]
		type PalletId: Get<PalletId>;
		type ValidatorId: Convert<
			<<Self as Config>::Validators as ValidatorSet<<Self as frame_system::Config>::AccountId>>::ValidatorId,
			Option<Self::AccountId>
		>;
		type ValidatorIdOf: Convert<
			Self::AccountId,
			Option<<<Self as Config>::ValidatorSet as ValidatorSet<<Self as frame_system::Config>::AccountId>>::ValidatorId>
		>;
		type Balance: Parameter +
			Member +
			AtLeast32BitUnsigned +
			Codec +
			Default +
			From<u128> +
			Into<u128> +
			Copy +
			MaybeSerializeDeserialize +
			Debug +
			MaxEncodedLen +
			TypeInfo +
			FixedPointOperand;

		#[pallet::constant]
		type TotalReward: Get<u32>;

		type Precision: Get<u32>;

		#[pallet::constant]
		type TotalMinutesPerYear: Get<u32>;
		type EraMinutes: Get<u32>;
		
		type RewardCurrency: LockableCurrency<
			Self::AccountId,
			Moment = BlockNumberFor<Self>,
			Balance = Self::Balance
		>;
	}



	/// The era reward which are distributed among the validator and nominator
	#[pallet::storage]
	#[pallet::getter(fn total_rewards)]
	pub type BeneficialRewardRecord<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		T::Balance,
		ValueQuery
	>;

	/// Specifics regarding the rewards distributed within the designated era of the validator
	#[pallet::storage]
	#[pallet::getter(fn validator_reward_accounts)]
	pub type ValidatorRewardAccounts<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		T::Balance,
		ValueQuery
	>;

	/// Specifics regarding the rewards distributed within the designated era of the nominator
	#[pallet::storage]
	#[pallet::getter(fn nominator_reward_accounts)]
	pub type NominatorRewardAccounts<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		T::Balance,
		ValueQuery
	>;

	/// Era reward accounts
	#[pallet::storage]
	#[pallet::getter(fn era_reward)]
	pub type EraRewardsVault<T: Config> = StorageValue<_, Vec<T::AccountId>>;

	// reward account
	#[pallet::storage]
	#[pallet::getter(fn reward_account)]
	pub type RewardAccount<T: Config> = StorageValue<_, T::AccountId>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// The rewards has been succesfully distributed
		Distributed {
			who: T::AccountId,
			balance: T::Balance,
		},
		/// The reward will be distributed after completely the era
		Rewarded {
			who: T::AccountId,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// No validator is present
		NoSuchValidator,
		/// No Reward is available for the author
		NoRewardAvailable,
		/// No Reward is available for the author
		AlreadyRewarded,
		/// No Such Nominator is present
		NoSuchNominator,
		/// Wait for the era to complete
		WaitTheEraToComplete,
	}


	#[pallet::genesis_config]
	#[derive(frame_support::DefaultNoBound)]
	pub struct GenesisConfig<T: Config> {
		#[serde(skip)]
		_config: sp_std::marker::PhantomData<T>,
	}
	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			let account = <Pallet<T>>::account_id();
			RewardAccount::<T>::put(account)

		}
	}

	impl<T: Config> Pallet<T> {
		pub fn account_id() -> T::AccountId {
			T::PalletId::get().into_account_truncating()
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(Weight::zero())]
		pub fn get_rewards(origin: OriginFor<T>, validator: T::AccountId) -> DispatchResult {
			ensure_signed(origin)?;
			Self::verify_validator(validator.clone())?;
			let mut era_reward_accounts = EraRewardsVault::<T>::get().unwrap_or_else(Vec::new);
			ensure!(!era_reward_accounts.contains(&validator), Error::<T>::WaitTheEraToComplete);
			era_reward_accounts.push(validator.clone());
			EraRewardsVault::<T>::put(era_reward_accounts);
			Self::deposit_event(Event::<T>::Rewarded { who: validator });
			Ok(())
		}
	}
}

impl<T: Config> Rewards<T::AccountId> for Pallet<T> {

	/// List of the validators who will recieve reward after the era
	fn payout_validators() -> Vec<T::AccountId> {
		let validators = EraRewardsVault::<T>::get().unwrap_or_else(Vec::new);
		validators
	}
	
	/// Distributing rewards to validators and nominators.
	fn claim_rewards(validator: T::AccountId) -> DispatchResult {
		let nominators = Self::check_nominators(validator.clone());
		if nominators.is_empty() {
			Self::distribute_validator_reward(validator.clone())?;
			Self::update_rewarded_accounts(validator.clone())?;
			return Ok(());
		}
		nominators.iter().for_each(|nominator| {
			let _ = Self::distribute_validator_reward(validator.clone());
			let _ = Self::distribute_nominator_reward(nominator.who.clone());
		});
		Self::update_rewarded_accounts(validator.clone())?;
		return Ok(());
	}

	/// Function for computing the rewards of validators and nominators at the end of each era
	fn calculate_reward() -> DispatchResult {
		let validators = T::Validators::validators();

		validators.iter().for_each(|validator_id| {
			let validator = T::ValidatorId::convert(validator_id.clone()).unwrap();
			let validator_points = Self::retrieve_validator_point(validator.clone());
			let era_reward = Self::calculate_era_reward();
<<<<<<< HEAD
			let total_reward = era_reward as f64;
=======
			let total_reward = (era_reward as f64) * (validators.len() as f64);
>>>>>>> 4f67521 (inflation removed and total supply constant (#117))
			let reward = Self::calculate_validator_reward(validator_points.into(), total_reward);
			let nominators = Self::check_nominators(validator.clone());
			if nominators.is_empty() {
				Self::allocate_validator_rewards(
					validator.clone(),
					Self::convert_float64_to_unsigned128(reward).into()
				);
				return;
			}
			let validator_commission = Self::validator_commission(validator.clone());
			let validator_share = ((reward as f64) * (validator_commission as f64)) / 100.0;
			let nominator_share = reward - validator_share;
			Self::allocate_validator_rewards(
				validator.clone(),
				Self::convert_float64_to_unsigned128(validator_share).into()
			);

			nominators.iter().for_each(|nominator| {
				let nominator_stake = nominator.value;
				let exposure = ErasStakers::<T>::get(Self::current_era(), validator.clone());
				let total_stake = exposure.total;
				let nominator_reward = Self::calculate_nominator_reward(
					nominator_stake.into(),
					total_stake.into(),
					nominator_share.into()
				);
				Self::allocate_nominator_reward(
					nominator.who.clone(),
					Self::convert_float64_to_unsigned128(nominator_reward).into()
				);
			});
		});
		Ok(())
	}
}

impl<T: Config> Pallet<T> {
	
	/// Transfer an amount to the accounts with respecting the `keep_alive` requirements.
	fn transfer(
		who: T::AccountId,
		dest: T::AccountId,
		amount: T::Balance,
		existence_requirement: ExistenceRequirement
	) -> DispatchResult {
		T::RewardCurrency::transfer(&who, &dest, amount, existence_requirement)?;
		Self::deposit_event(Event::Distributed { who: dest, balance: amount });
		Ok(())
	}

	/// Validation of an account to determine its validator status.
	fn verify_validator(account: T::AccountId) -> DispatchResult {
		let all_validators = T::DataProvider::electable_targets(
			DataProviderBounds::default()
		).unwrap();
		let val = all_validators
			.iter()
			.any(|c| T::ValidatorIdOf::convert(account.clone()) == Some(c.clone()));
		if val {
			return Ok(());
		} else {
			return Err(Error::<T>::NoSuchValidator.into());
		}
	}

	/// Calculates the commission for the validator.
	fn validator_commission(validator: T::AccountId) -> u32{
		let validator_prefs = Validators::<T>::get(validator.clone());
		let validator_commission = validator_prefs.commission.deconstruct();
		let precision: u32 = 7;
		let scaled_commission: u32 = validator_commission / (10u32).pow(precision);
		scaled_commission
	}

	/// Update the list of validators who have already been rewarded.
	fn update_rewarded_accounts(account: T::AccountId) -> DispatchResult {
		let mut era_reward_accounts = EraRewardsVault::<T>::get().unwrap_or_else(Vec::new);
		if let Some(index) = era_reward_accounts.iter().position(|a| a == &account.clone()) {
			era_reward_accounts.remove(index);
		}
		EraRewardsVault::<T>::put(era_reward_accounts);
		Ok(())
	}

	/// Compute the total era reward 
	pub fn calculate_era_reward() -> f64 {
		let total_minutes_per_year = T::TotalMinutesPerYear::get();
		let era_minutes = T::EraMinutes::get();
		let era = total_minutes_per_year / era_minutes;
		let total_reward = T::TotalReward::get();
		let era_reward = total_reward / era;
		era_reward.into()
	}

	/// Compute the reward of the nominator
	pub fn calculate_nominator_reward(share: u128, total_stake: u128, reward: f64) -> f64 {
		let precision = T::Precision::get();
		let scaled_share = share / (10u128).pow(precision);
		let scaled_total_stake: u64 = (total_stake / (10u128).pow(precision)) as u64;
		let division: f64 = ((scaled_share as f64) / (scaled_total_stake as f64)) as f64;
		let total_reward = division * reward;
		total_reward
	}

	/// Converts a floating-point number to an unsigned 128-bit integer.
	pub fn convert_float64_to_unsigned128(value: f64) -> u128 {
		let precision = T::Precision::get();
		let multiplier = (10u128).pow(precision);
		let number = (value * (multiplier as f64)) as u128;
		number
	}

	/// Allocates rewards to the specified validator.
	fn allocate_validator_rewards(validator: T::AccountId, reward: T::Balance) {
		let earlier_reward = ValidatorRewardAccounts::<T>::get(validator.clone());
		let new_individual_reward = reward + earlier_reward;
		ValidatorRewardAccounts::<T>::insert(validator.clone(), new_individual_reward);
	}

	/// Allocates rewards to the specified nominator.
	fn allocate_nominator_reward(nominator: T::AccountId, reward: T::Balance) {
		let earlier_reward = NominatorRewardAccounts::<T>::get(nominator.clone());
		let new_individual_reward = reward + earlier_reward;
		NominatorRewardAccounts::<T>::insert(nominator.clone(), new_individual_reward);
	}

	/// Retrieves the points of the validator.
	fn retrieve_validator_point(account: T::AccountId) -> u32 {
		let era_reward_points = <ErasRewardPoints<T>>::get(Self::active_era());
		let validator_points = era_reward_points.individual.get(&account).unwrap_or(&0);
		*validator_points
	}

	/// Distributes rewards to the specified validator.
	fn distribute_validator_reward(account: T::AccountId) -> DispatchResult {
		let reward = ValidatorRewardAccounts::<T>::get(account.clone());
		Self::transfer(Self::account_id(), account.clone(), reward, KeepAlive)?;
		ValidatorRewardAccounts::<T>::remove(account.clone());
		
		//BeneficialRewardRecord::<T>::insert(account.clone(), reward);
		Self::store_reward_received(account,reward);

		Ok(())
	}

	/// Distributes rewards to the specified nominator.
	fn distribute_nominator_reward(account: T::AccountId) -> DispatchResult {
		let reward = NominatorRewardAccounts::<T>::get(account.clone());
		if reward.is_zero(){
			return Ok(());
		}
		Self::transfer(Self::account_id(), account.clone(), reward, KeepAlive)?;
		// let staking_account = T::LiquidStakeVault::staking_account();
		// if account != staking_account {
		// 	NominatorRewardAccounts::<T>::remove(account.clone());
		// }
	//	BeneficialRewardRecord::<T>::insert(account.clone(), reward);
	  Self::store_reward_received(account,reward);
       Ok(())
	}

	/// Current era index
	fn current_era() -> u32 {
		CurrentEra::<T>::get().unwrap_or(0)
	}

	/// Active era index
	fn active_era() -> u32 {
		let active_era = pallet_staking::Pallet::<T>::active_era().unwrap();
		let era = active_era.index;
		era
	}
/// Store the received reward for a specific account.
fn store_reward_received(account:T::AccountId,reward:T::Balance){
	let earlier_reward = BeneficialRewardRecord::<T>::get(account.clone());
	let new_reward = reward + earlier_reward;
	BeneficialRewardRecord::<T>::insert(account.clone(), new_reward);
}

	/// Compute the reward of the validator
	fn calculate_validator_reward(validator_points: u32, era_reward: f64) -> f64 {
		let era_reward_points = <ErasRewardPoints<T>>::get(Self::active_era());
		let total_points = era_reward_points.total as u32;
		let reward = ((validator_points as f64) / (total_points as f64)) * (era_reward as f64);
		reward
	}

	/// Determine whether the validator has nominators in the current era.
	fn check_nominators(
		validator: T::AccountId
	) -> Vec<IndividualExposure<T::AccountId, <T as pallet_staking::Config>::CurrencyBalance>> {
		let exposure = ErasStakers::<T>::get(Self::current_era(), validator.clone());
		let nominators = exposure.others;
		nominators
	}
}
