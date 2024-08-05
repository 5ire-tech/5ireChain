use crate::mock::*;
use crate::Error;
use crate::ValidatorRewardAccounts;
use frame_support::{assert_ok, assert_noop};

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
