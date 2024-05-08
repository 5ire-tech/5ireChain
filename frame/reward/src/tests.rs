use crate::mock::*;
use frame_support::assert_ok;

#[test]
fn get_rewards_should_work() {
	ExtBuilder::default().build_and_execute(|| {
		start_session(1);

		assert_eq!(active_era(), 0);
		assert_eq!(RewardBalance::free_balance(11), 1000);
		assert_ok!(Reward::get_rewards(who(11), 11));
	});
}
