use crate::mock::*;
use crate::Error;
use crate::{Rewards,ValidatorRewardAccounts,NominatorRewardAccounts,EraReward};
use frame_support::{assert_ok, assert_noop};
use frame_support::traits::Currency;
use frame_system::Event;
use sp_runtime::Perbill;

pub const VALIDATOR: u64 = 11;
pub const NOMINATOR: u64 = 22; 
pub const USER: u64 = 1;
pub const USER_2: u64 = 2;

pub fn add_reward_balance() {
	Balances::deposit_creating(&Reward::account_id(), 15000000);
}

#[test]
fn get_rewards_should_work() {
	ExtBuilder::default().build_and_execute(|| {
		start_session(1);
		ValidatorRewardAccounts::<Test>::insert(VALIDATOR, 1000);
        add_reward_balance();
		assert_eq!(active_era(), 0);
		assert_eq!(RewardBalance::free_balance(VALIDATOR), 1000);
		assert_ok!(Reward::get_rewards(who(VALIDATOR), VALIDATOR));
	});
}

#[test]
fn anyone_can_call_rewards_should_work() {
	ExtBuilder::default().build_and_execute(|| {
		start_session(1);
		assert_eq!(active_era(), 0);
        add_reward_balance();
		ValidatorRewardAccounts::<Test>::insert(VALIDATOR, 1000);
		assert_ok!(Reward::get_rewards(who(USER), VALIDATOR));
	});
}

#[test]
fn get_rewards_by_not_validator_should_not_work() {
	ExtBuilder::default().build_and_execute(|| {
		start_session(1);
		assert_eq!(active_era(), 0);
		assert_noop!(Reward::get_rewards(who(USER), USER_2), Error::<Test>::NoReward);
	});
}

#[test]
fn get_multiple_rewards_in_the_same_era_should_not_work() {
	ExtBuilder::default().build_and_execute(|| {
		start_session(1);
		assert_eq!(active_era(), 0);
        add_reward_balance();
		ValidatorRewardAccounts::<Test>::insert(VALIDATOR, 1000);
        assert_ok!(Reward::get_rewards(who(USER), VALIDATOR));
		assert_noop!(Reward::get_rewards(who(USER), VALIDATOR), Error::<Test>::WaitTheEraToComplete);
	});
}

#[test]
fn nominator_receiving_reward() {
    ExtBuilder::default().build_and_execute(|| {
        start_session(1);
        assert_eq!(active_era(), 0);
        let nominator_reward :u128 = 500;
		let validator_reward: u128 = 1000; 

        let earlier_nominator_balance =RewardBalance::free_balance(NOMINATOR);
        let earlier_validator_balance =RewardBalance::free_balance(VALIDATOR);

        ValidatorRewardAccounts::<Test>::insert(VALIDATOR, validator_reward);
        NominatorRewardAccounts::<Test>::insert(VALIDATOR, NOMINATOR, nominator_reward);
        EraReward::<Test>::insert(VALIDATOR,vec![NOMINATOR]);

        let _ = Balances::deposit_creating(&Reward::account_id(), 15000000);
		let reward_account_balance_before =RewardBalance::free_balance(Reward::account_id());
		assert_eq!(reward_account_balance_before , 15000000); 
        assert_ok!(Reward::get_rewards(who(USER), VALIDATOR));
        let _=  Reward::claim_rewards(VALIDATOR);

		// Validator balance check
        let validator_balance =RewardBalance::free_balance(VALIDATOR);
        assert_eq!(validator_balance, earlier_validator_balance + validator_reward); 

		// Nominator balance check 
        let nominator_balance =RewardBalance::free_balance(NOMINATOR);
        assert_eq!(nominator_balance,earlier_nominator_balance + nominator_reward); 

		// Reward balance check 
		let reward_account_balance_after =RewardBalance::free_balance(Reward::account_id());
		assert_eq!(reward_account_balance_after , 15000000 - validator_reward - nominator_reward); 

    }); 
}

#[test]
fn accumulated_rewards_over_multiple_eras() {
    ExtBuilder::default().build_and_execute(|| {
        start_session(1);
        assert_eq!(active_era(), 0);
        add_reward_balance();

        let mut total_validator_reward = 0;
		let earlier_validator_balance =RewardBalance::free_balance(VALIDATOR);
        let mut total_nominator_reward = 0;

        for era in 0..3 {
            let validator_reward: u128 = 1000 * (era + 1);
            let nominator_reward: u128 = 500 * (era + 1);

			ValidatorRewardAccounts::<Test>::mutate(VALIDATOR.clone(), |earlier_reward| {
				*earlier_reward += validator_reward;
			});
			NominatorRewardAccounts::<Test>::mutate(VALIDATOR.clone(), NOMINATOR,|earlier_reward| {
				*earlier_reward += nominator_reward;
			});
            EraReward::<Test>::insert(VALIDATOR, vec![NOMINATOR]);

            total_validator_reward += validator_reward;
            total_nominator_reward += nominator_reward;
        }

		assert_ok!(Reward::get_rewards(who(USER), VALIDATOR));
        let _ = Reward::claim_rewards(VALIDATOR);

        assert_eq!(RewardBalance::free_balance(VALIDATOR), earlier_validator_balance + total_validator_reward);
        assert_eq!(RewardBalance::free_balance(NOMINATOR), total_nominator_reward);
    });
}

#[test]
fn reward_distribution_with_zero_commission() {
    ExtBuilder::default().build_and_execute(|| {
        start_session(1);
        assert_eq!(active_era(), 0);
        add_reward_balance();

        let validator_reward: u128 = 1000;
		let earlier_validator_balance =RewardBalance::free_balance(VALIDATOR);
        let nominator_reward: u128 = 500;
        let commission = Perbill::from_percent(0);

        ValidatorRewardAccounts::<Test>::insert(VALIDATOR, validator_reward);
        NominatorRewardAccounts::<Test>::insert(VALIDATOR, NOMINATOR, nominator_reward);
        EraReward::<Test>::insert(VALIDATOR, vec![NOMINATOR]);

        let _ = Staking::set_min_commission(who(VALIDATOR),commission);
        let _ = Reward::claim_rewards(VALIDATOR);

        // Check the balances
        assert_eq!(RewardBalance::free_balance(VALIDATOR), earlier_validator_balance + validator_reward);
        assert_eq!(RewardBalance::free_balance(NOMINATOR), nominator_reward);
    });
}