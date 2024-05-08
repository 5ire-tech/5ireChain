use crate::mock::*;
use frame_support::assert_ok;
use frame_support::traits::Currency;

type AccountIdOf<Test> = <Test as frame_system::Config>::AccountId;




fn transfer_balance(){
    System::set_block_number(1);
    let _ =  RewardBalance::deposit_creating(&2,150000000000);
    let _ = RewardBalance::transfer(who(2), 3, 15000);
}

#[test]
fn get_rewards(){
    new_test_ext().execute_with(||{
        System::set_block_number(1);
        transfer_balance();
        //assert_ok!(Reward::get_rewards(RuntimeOrigin::signed(account(1))));
    });
}