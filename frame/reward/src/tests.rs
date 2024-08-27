use crate::mock::*;
use crate::Error;
use crate::{Rewards,ValidatorRewardAccounts,NominatorRewardAccounts,EraReward};
use frame_support::{assert_ok, assert_noop};
use frame_support::traits::Currency;

pub const VALIDATOR: u64 = 11;
pub const NOMINATOR: u64 = 22; 
pub const USER: u64 = 1;
pub const USER_2: u64 = 2;


#[test]
fn get_rewards_should_work() {
	ExtBuilder::default().build_and_execute(|| {
		start_session(1);
		ValidatorRewardAccounts::<Test>::insert(VALIDATOR, 1000);
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