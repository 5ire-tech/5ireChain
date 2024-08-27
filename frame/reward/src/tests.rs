use crate::mock::*;
use crate::Error;
use crate::{Rewards,ValidatorRewardAccounts,NominatorRewardAccounts,EraReward};
use frame_support::{assert_ok, assert_noop};
use frame_support::traits::Currency;
#[test]
fn get_rewards_should_work() {
	ExtBuilder::default().build_and_execute(|| {
		start_session(1);
		ValidatorRewardAccounts::<Test>::insert(11, 1000);
		assert_eq!(active_era(), 0);
		assert_eq!(RewardBalance::free_balance(11), 1000);
		assert_ok!(Reward::get_rewards(who(11), 11));
	});
}

#[test]
fn anyone_can_call_rewards_should_work() {
	ExtBuilder::default().build_and_execute(|| {
		start_session(1);
		assert_eq!(active_era(), 0);
		ValidatorRewardAccounts::<Test>::insert(11, 1000);
		assert_ok!(Reward::get_rewards(who(1), 11));
	});
}

#[test]
fn get_rewards_by_not_validator_should_not_work() {
	ExtBuilder::default().build_and_execute(|| {
		start_session(1);
		assert_eq!(active_era(), 0);
        let normal_user:u64 = 2;

		assert_noop!(Reward::get_rewards(who(1), normal_user), Error::<Test>::NoReward);
	});
}

#[test]
fn get_multiple_rewards_in_the_same_era_should_not_work() {
	ExtBuilder::default().build_and_execute(|| {
		start_session(1);
		assert_eq!(active_era(), 0);
		ValidatorRewardAccounts::<Test>::insert(11, 1000);
        assert_ok!(Reward::get_rewards(who(1), 11));
		assert_noop!(Reward::get_rewards(who(1), 11), Error::<Test>::WaitTheEraToComplete);
	});
}

#[test]
fn nominator_receiving_reward() {
    ExtBuilder::default().build_and_execute(|| {
        start_session(1);
        assert_eq!(active_era(), 0);

        let validator: u64 = 11;
        let nominator: u64 = 21;
        let nominator_reward :u128 = 500;
		let validator_reward: u128 = 1000; 

        let earlier_nominator_balance =RewardBalance::free_balance(21);
        let earlier_validator_balance =RewardBalance::free_balance(11);

        ValidatorRewardAccounts::<Test>::insert(validator, validator_reward);
        NominatorRewardAccounts::<Test>::insert(validator, nominator, nominator_reward);
        EraReward::<Test>::insert(validator,vec![nominator]);

        let _ = Balances::deposit_creating(&Reward::account_id(), 15000000);
		let reward_account_balance_before =RewardBalance::free_balance(Reward::account_id());
		assert_eq!(reward_account_balance_before , 15000000); 
        assert_ok!(Reward::get_rewards(who(1), validator));
        let _=  Reward::claim_rewards(validator);

		// Validator balance check
        let validator_balance =RewardBalance::free_balance(21);
        assert_eq!(validator_balance, earlier_validator_balance + validator_reward); 

		// Nominator balance check 
        let nominator_balance =RewardBalance::free_balance(21);
        assert_eq!(nominator_balance,earlier_nominator_balance + nominator_reward); 

		// Reward balance check 
		let reward_account_balance_after =RewardBalance::free_balance(Reward::account_id());
		assert_eq!(reward_account_balance_after , 15000000 - validator_reward - nominator_reward); 

    }); 
}