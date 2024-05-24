use crate::mock::*;
use frame_support::assert_ok;
use crate::AuthorBlockList;
use frame_support::traits::Currency;

type AccountIdOf<Test> = <Test as frame_system::Config>::AccountId;

fn account(id: u8) -> AccountIdOf<Test> {
	[id; 20].into()
}

fn set_validator(){
    let validator = pallet_staking::ValidatorPrefs;
}

fn transfer_balance(){
    System::set_block_number(1);
    let _ =  RewardBalance::deposit_creating(&account(2),150000000000);
    let _ = RewardBalance::transfer(RuntimeOrigin::signed(account(2)), account(3), 15000);
    AuthorBlockList::<Test>::insert(account(1), 1)
}

#[test]
fn get_rewards(){
    new_test_ext().execute_with(||{
        System::set_block_number(1);
        transfer_balance();
        assert_ok!(crate::Pallet::<Test>::get_rewards(RuntimeOrigin::signed(account(1))));
    });
}