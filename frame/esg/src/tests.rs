use fp_account::AccountId20;
use sp_runtime::DispatchError;
pub use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, WeakBoundedVec};

const MAX_ESG_SCORE: u16 = 100;

#[allow(non_snake_case)]
struct Addr {
	ROOT: AccountId20,
	ALICE: AccountId20,
	SUDO_ORACLE: AccountId20,
	SUDO_ORACLE_2: AccountId20,
	NON_SUDO_ORACLE: AccountId20,
	DUMMY_SUDO_ORACLE: AccountId20,
	NON_SUDO_ORACLE_2: AccountId20,
	NON_SUDO_ORACLE_6: AccountId20,
	NON_SUDO_ORACLE_7: AccountId20,
	NON_SUDO_ORACLE_8: AccountId20,
	NON_SUDO_ORACLE_9: AccountId20,
	NON_SUDO_ORACLE_10: AccountId20,
}

impl Default for Addr {
	fn default() -> Self {
		Self {
			ROOT: AccountId20::from([0u8; 20]),
			ALICE: AccountId20::from([4u8; 20]),
			SUDO_ORACLE: AccountId20::from([0u8; 20]),
			SUDO_ORACLE_2: AccountId20::from([1u8; 20]),
			NON_SUDO_ORACLE: AccountId20::from([2u8; 20]),
			DUMMY_SUDO_ORACLE: AccountId20::from([5u8; 20]),
			NON_SUDO_ORACLE_2: AccountId20::from([3u8; 20]),
			NON_SUDO_ORACLE_6: AccountId20::from([6u8; 20]),
			NON_SUDO_ORACLE_7: AccountId20::from([7u8; 20]),
			NON_SUDO_ORACLE_8: AccountId20::from([8u8; 20]),
			NON_SUDO_ORACLE_9: AccountId20::from([9u8; 20]),
			NON_SUDO_ORACLE_10: AccountId20::from([10u8; 20]),
		}
	}
}

#[test]
#[allow(non_snake_case)]
fn it_must_register_oracles_new() {
	new_test_ext().execute_with(|| {
		let addr = Addr::default();
		
		assert_ok!(Esg::register_an_oracle(RuntimeOrigin::root(), addr.SUDO_ORACLE, true));
		assert_ok!(Esg::register_an_oracle(
			RuntimeOrigin::signed(addr.SUDO_ORACLE),
			addr.NON_SUDO_ORACLE_6,
			false
		));
		assert_ok!(Esg::register_an_oracle(
			RuntimeOrigin::signed(addr.SUDO_ORACLE),
			addr.NON_SUDO_ORACLE_8,
			false
		));
		assert_ok!(Esg::register_an_oracle(
			RuntimeOrigin::signed(addr.SUDO_ORACLE),
			addr.NON_SUDO_ORACLE_9,
			false
		));
		assert_ok!(Esg::register_an_oracle(
			RuntimeOrigin::signed(addr.SUDO_ORACLE),
			addr.NON_SUDO_ORACLE_10,
			false
		));
		assert_ok!(Esg::register_an_oracle(
			RuntimeOrigin::signed(addr.SUDO_ORACLE),
			addr.NON_SUDO_ORACLE_7,
			false
		));

		assert_ok!(Esg::deregister_an_oracle(RuntimeOrigin::root(), addr.NON_SUDO_ORACLE_7, false));
	});
}

#[test]
fn it_must_register_oracles() {
	new_test_ext().execute_with(|| {
		let addr = Addr::default();

		System::set_block_number(1);
		// registering a sudo oracle with root
		// Check extrinsic should work for sudo oracle
		// SUDO_ORACLE is sudo oracle which is assigned by Sudo key
		assert_ok!(Esg::register_an_oracle(RuntimeOrigin::root(), addr.SUDO_ORACLE, true));

		// Check sudo oracle
		assert!(Esg::get_oracle_sudo().contains(&addr.SUDO_ORACLE));

		// check `NewOracleRegistered` was trigered with expected data
		System::assert_last_event(RuntimeEvent::Esg(crate::Event::NewOracleRegistered {
			is_sudo: true,
			oracle: addr.SUDO_ORACLE,
		}));

		System::set_block_number(1);
		// registering a non-sudo oracle with root
		// Check extrinsic should work for non sudo oracle
		// due to sudo key -> NON_SUDO_ORACLE become SUDO ORACLE with false condition

		assert_ok!(Esg::register_an_oracle(RuntimeOrigin::root(), addr.NON_SUDO_ORACLE, false));

		assert!(Esg::get_oracle_nsudo().contains(&addr.NON_SUDO_ORACLE));

		// check `NewOracleRegistered` was trigered with expected data
		System::assert_last_event(RuntimeEvent::Esg(crate::Event::NewOracleRegistered {
			is_sudo: false,
			oracle: addr.NON_SUDO_ORACLE,
		}));

		System::set_block_number(1);

		// Sudo oracle assign to new sudo oracle
		assert_ok!(Esg::register_an_oracle(
			RuntimeOrigin::signed(addr.SUDO_ORACLE),
			addr.SUDO_ORACLE_2,
			true
		));

		assert!(Esg::get_oracle_sudo().contains(&addr.SUDO_ORACLE_2));

		// check `NewOracleRegistered` was trigered with expected data
		System::assert_last_event(RuntimeEvent::Esg(crate::Event::NewOracleRegistered {
			is_sudo: true,
			oracle: addr.SUDO_ORACLE_2,
		}));

		System::set_block_number(1);

		// Sudo oracle assign to another non sudo oracle
		assert_ok!(Esg::register_an_oracle(
			RuntimeOrigin::signed(addr.SUDO_ORACLE),
			addr.NON_SUDO_ORACLE_2,
			false
		));

		assert!(Esg::get_oracle_nsudo().contains(&addr.NON_SUDO_ORACLE_2));

		// check `NewOracleRegistered` was trigered with expected data
		System::assert_last_event(RuntimeEvent::Esg(crate::Event::NewOracleRegistered {
			is_sudo: false,
			oracle: addr.NON_SUDO_ORACLE_2,
		}));
	});
}

#[test]
fn it_must_fail_to_register_existent_oracle_again() {
	new_test_ext().execute_with(|| {
		let addr = Addr::default();

		System::set_block_number(1);
		// registering a sudo oracle with root
		// Check extrinsic should work for sudo oracle
		// SUDO_ORACLE is sudo oracle which is assigned by Sudo key
		assert_ok!(Esg::register_an_oracle(RuntimeOrigin::root(), addr.SUDO_ORACLE, true));

		// Check sudo oracle
		assert!(Esg::get_oracle_sudo().contains(&addr.SUDO_ORACLE));

		// check `NewOracleRegistered` was trigered with expected data
		System::assert_last_event(RuntimeEvent::Esg(crate::Event::NewOracleRegistered {
			is_sudo: true,
			oracle: addr.SUDO_ORACLE,
		}));

		assert_noop!(
			Esg::register_an_oracle(RuntimeOrigin::root(), addr.SUDO_ORACLE, true),
			Error::<Test>::OracleRegisteredAlready
		);

		System::set_block_number(1);
		// registering a non-sudo oracle with root
		// Check extrinsic should work for non sudo oracle
		// due to sudo key -> NON_SUDO_ORACLE become SUDO ORACLE with false condition

		assert_ok!(Esg::register_an_oracle(RuntimeOrigin::root(), addr.NON_SUDO_ORACLE, false));

		assert!(Esg::get_oracle_nsudo().contains(&addr.NON_SUDO_ORACLE));

		// check `NewOracleRegistered` was trigered with expected data
		System::assert_last_event(RuntimeEvent::Esg(crate::Event::NewOracleRegistered {
			is_sudo: false,
			oracle: addr.NON_SUDO_ORACLE,
		}));

		assert_noop!(
			Esg::register_an_oracle(RuntimeOrigin::root(), addr.NON_SUDO_ORACLE, false),
			Error::<Test>::OracleRegisteredAlready
		);
	});
}

#[test]
fn it_must_not_register_oracles_with_non_sudo_oracle_non_root_or_unsigned_origins() {
	new_test_ext().execute_with(|| {
		let addr = Addr::default();

		System::set_block_number(1);
		// expect `NotSigned` error
		assert_noop!(
			Esg::register_an_oracle(RuntimeOrigin::none(), addr.ALICE, true),
			Error::<Test>::NotSigned
		);

		// expect `CallerNotRootOrSudoOracle` error
		assert_noop!(
			Esg::register_an_oracle(RuntimeOrigin::signed(addr.ALICE), addr.ALICE, true),
			Error::<Test>::CallerNotRootOrSudoOracle
		);

		// expect `CallerNotRootOrSudoOracle` error
		assert_noop!(
			Esg::register_an_oracle(RuntimeOrigin::signed(addr.ALICE), addr.ALICE, false),
			Error::<Test>::CallerNotRootOrSudoOracle
		);

		// expect `CallerNotRootOrSudoOracle` error
		assert_noop!(
			Esg::register_an_oracle(RuntimeOrigin::signed(addr.ALICE), addr.DUMMY_SUDO_ORACLE, true),
			Error::<Test>::CallerNotRootOrSudoOracle
		);

		// expect `CallerNotRootOrSudoOracle` error
		assert_noop!(
			Esg::register_an_oracle(RuntimeOrigin::signed(addr.ALICE), addr.DUMMY_SUDO_ORACLE, false),
			Error::<Test>::CallerNotRootOrSudoOracle
		);
	});
}

#[test]
fn it_must_deregister_oracles_with_root_only() {
	new_test_ext().execute_with(|| {
		let addr = Addr::default();

		// -------------- deregistering sudo oracles ---------------

		System::set_block_number(1);

		// registering a sudo oracle with root
		assert_ok!(Esg::register_an_oracle(RuntimeOrigin::root(), addr.SUDO_ORACLE, true));

		// Check sudo oracle added
		assert!(Esg::get_oracle_sudo().contains(&addr.SUDO_ORACLE));

		// check `NewOracleRegistered` was trigered with expected data
		System::assert_last_event(RuntimeEvent::Esg(crate::Event::NewOracleRegistered {
			is_sudo: true,
			oracle: addr.SUDO_ORACLE,
		}));

		assert_ok!(Esg::deregister_an_oracle(RuntimeOrigin::root(), addr.SUDO_ORACLE, true));

		// Check sudo oracle removed
		assert!(!Esg::get_oracle_sudo().contains(&addr.SUDO_ORACLE));

		// check `NewOracleRegistered` was trigered with expected data
		System::assert_last_event(RuntimeEvent::Esg(crate::Event::OracleDeRegistered {
			is_sudo: true,
			oracle: addr.SUDO_ORACLE,
		}));

		// lets check for non-root signed txn

		System::set_block_number(1);

		// registering a sudo oracle with root
		assert_ok!(Esg::register_an_oracle(RuntimeOrigin::root(), addr.SUDO_ORACLE, true));

		// check `NewOracleRegistered` was trigered with expected data
		System::assert_last_event(RuntimeEvent::Esg(crate::Event::NewOracleRegistered {
			is_sudo: true,
			oracle: addr.SUDO_ORACLE,
		}));

		// expect `BadOrigin` error
		assert_noop!(
			Esg::deregister_an_oracle(RuntimeOrigin::signed(addr.ALICE), addr.SUDO_ORACLE, true),
			DispatchError::BadOrigin
		);

		// --------------- deregistering non-sudo oracles ----------------

		System::set_block_number(1);

		// registering a non-sudo oracle with root
		assert_ok!(Esg::register_an_oracle(RuntimeOrigin::root(), addr.NON_SUDO_ORACLE, false));

		// check `NewOracleRegistered` was trigered with expected data
		System::assert_last_event(RuntimeEvent::Esg(crate::Event::NewOracleRegistered {
			is_sudo: false,
			oracle: addr.NON_SUDO_ORACLE,
		}));

		// deregistering a non-sudo oracle with root
		assert_ok!(Esg::deregister_an_oracle(RuntimeOrigin::root(), addr.NON_SUDO_ORACLE, false));

		// check `OracleDeRegistered` was trigered with expected data
		System::assert_last_event(RuntimeEvent::Esg(crate::Event::OracleDeRegistered {
			is_sudo: false,
			oracle: addr.NON_SUDO_ORACLE,
		}));
	});
}

#[test]
fn it_must_fail_to_deregister_nonexistent_oracle() {
	new_test_ext().execute_with(|| {
		let addr = Addr::default();

		// -------------- deregistering sudo oracles ---------------

		System::set_block_number(1);

		// registering a sudo oracle with root
		assert_ok!(Esg::register_an_oracle(RuntimeOrigin::root(), addr.SUDO_ORACLE, true));

		// Check sudo oracle added
		assert!(Esg::get_oracle_sudo().contains(&addr.SUDO_ORACLE));

		// check `NewOracleRegistered` was trigered with expected data
		System::assert_last_event(RuntimeEvent::Esg(crate::Event::NewOracleRegistered {
			is_sudo: true,
			oracle: addr.SUDO_ORACLE,
		}));

		assert_ok!(Esg::deregister_an_oracle(RuntimeOrigin::root(), addr.SUDO_ORACLE, true));

		// Check sudo oracle removed
		assert!(!Esg::get_oracle_sudo().contains(&addr.SUDO_ORACLE));

		// check `NewOracleRegistered` was trigered with expected data
		System::assert_last_event(RuntimeEvent::Esg(crate::Event::OracleDeRegistered {
			is_sudo: true,
			oracle: addr.SUDO_ORACLE,
		}));

		// lets check for non-root signed txn

		System::set_block_number(2);

		// try deregistering the oracle which doesn't exist
		assert_noop!(
			Esg::deregister_an_oracle(RuntimeOrigin::root(), addr.SUDO_ORACLE, true),
			Error::<Test>::OracleNotExist
		);
	});
}

#[test]

fn it_must_not_allow_upload_from_a_non_oracle() {
	new_test_ext().execute_with(|| {
		let addr = Addr::default();

		let data = r#"[
			{
				"score":"3599760637",
				"account":"0x82a0EcfDd3174bEF5D5eA452e15219A52bf6161f"
			},
			{
				"score":"447537718",
				"account":"0xBa08C49f377a4F01c65340F431B5C65D71B972a9"
			}
		]"#;

		// expect `CallerNotAnOracle` error
		assert_noop!(
			Esg::upsert_esg_scores(
				RuntimeOrigin::signed(addr.ALICE),
				(WeakBoundedVec::try_from(data.as_bytes().to_vec())).unwrap()
			),
			Error::<Test>::CallerNotAnOracle
		);
	});
}

#[test]
fn it_must_not_accept_wrongly_formatted_input() {
	new_test_ext().execute_with(|| {
		let addr = Addr::default();

		System::set_block_number(1);
		let complete_invalid_data = r#"@01234abcd"#;

		let non_array_data = r#"
			{
				"score":"3599760637",
				"account":"0x82a0EcfDd3174bEF5D5eA452e15219A52bf6161f"
			},
			{
				"score":"447537718",
				"account":"0xBa08C49f377a4F01c65340F431B5C65D71B972a9"
			}"#;

		let data_with_quotes_missing = r#"[
			{
				"score":"3599760637",
				account":"0x82a0EcfDd3174bEF5D5eA452e15219A52bf6161f"
			},
			{
				"score":"447537718",
				"account":"0xBa08C49f377a4F01c65340F431B5C65D71B972a9"
			}
		]"#;

		let data_with_broken_key_value = r#"[
			{
				"score":"3599760637",
				"account":
			},
			{
				"score":"447537718",
				"account":"0xBa08C49f377a4F01c65340F431B5C65D71B972a9"
			}
		]"#;

		assert_ok!(Esg::register_an_oracle(RuntimeOrigin::root(), addr.SUDO_ORACLE, true));

		// check `NewOracleRegistered` was trigered with expected data
		System::assert_last_event(RuntimeEvent::Esg(crate::Event::NewOracleRegistered {
			is_sudo: true,
			oracle: addr.SUDO_ORACLE,
		}));

		// expect `InvalidJson` error
		assert_noop!(
			Esg::upsert_esg_scores(
				RuntimeOrigin::signed(addr.SUDO_ORACLE),
				(WeakBoundedVec::try_from(complete_invalid_data.as_bytes().to_vec())).unwrap()
			),
			Error::<Test>::InvalidJson
		);

		// expect `InvalidJson` error
		assert_noop!(
			Esg::upsert_esg_scores(
				RuntimeOrigin::signed(addr.SUDO_ORACLE),
				(WeakBoundedVec::try_from(non_array_data.as_bytes().to_vec())).unwrap()
			),
			Error::<Test>::InvalidJson
		);

		// expect `InvalidJson` error
		assert_noop!(
			Esg::upsert_esg_scores(
				RuntimeOrigin::signed(addr.SUDO_ORACLE),
				(WeakBoundedVec::try_from(data_with_quotes_missing.as_bytes().to_vec())).unwrap()
			),
			Error::<Test>::InvalidJson
		);

		// expect `InvalidJson` error
		assert_noop!(
			Esg::upsert_esg_scores(
				RuntimeOrigin::signed(addr.SUDO_ORACLE),
				(WeakBoundedVec::try_from(data_with_broken_key_value.as_bytes().to_vec())).unwrap()
			),
			Error::<Test>::InvalidJson
		);
	});
}

#[test]
fn it_must_upload_all_valid_esg_data_with_no_skipping() {
	new_test_ext().execute_with(|| {
		let addr = Addr::default();

		// to generate an event data
		System::set_block_number(1);
		let data = r#"[
			{
				"score":"12",
				"account":"0x82a0EcfDd3174bEF5D5eA452e15219A52bf6161f"
			},
			{
				"score":"40",
				"account":"0xBa08C49f377a4F01c65340F431B5C65D71B972a9"
			}
		]"#;

		assert_ok!(Esg::register_an_oracle(RuntimeOrigin::root(), addr.SUDO_ORACLE, true));

		assert_ok!(Esg::upsert_esg_scores(
			RuntimeOrigin::signed(addr.SUDO_ORACLE),
			(WeakBoundedVec::try_from(data.as_bytes().to_vec())).unwrap()
		));

		let company1 = Esg::hexstr2acc_id20("82a0EcfDd3174bEF5D5eA452e15219A52bf6161f");
		let company2 = Esg::hexstr2acc_id20("Ba08C49f377a4F01c65340F431B5C65D71B972a9");

		assert_eq!(Esg::get_score_of(company1), 12);
		assert_eq!(Esg::get_score_of(company2), 40);

		// event `ESGStored` must have got triggered with expected data
		System::assert_last_event(RuntimeEvent::Esg(crate::Event::ESGStored { caller: addr.ROOT }));
	});
}

#[test]
fn it_must_skip_for_invalid_addresses_in_esg_data() {
	new_test_ext().execute_with(|| {
		let addr = Addr::default();

		System::set_block_number(1);
		// invalid addresses at indexes 0, 2, 3, 5
		let wrong_address_data = r#"[
			{
				"score":"3599760637",
				"account":"0x82a0EcfDd3174bEF5D5eA452e15219A52bf6161f"
			},
			{
				"score":"3599760637",
				"account":"0xBa08C49f377a4F01c65340F431B5C65D71B972a"
			},
			{
				"score":"3599760637",
				"account":"i am an invalid address, as u can c!"
			},
			{
				"score":"3599760637",
				"account":"-1"
			},
			{
				"score":"99",
				"account":"0x25Db9D98e4Ab6af68ac7173f580c444155B7b5C8"
			},
			{
				"score":"3599760637",
				"account":"12345678910111213141516171819202122232425262728293"
			}
		]"#;
		// register sudo as an oracle
		assert_ok!(Esg::register_an_oracle(RuntimeOrigin::root(), addr.SUDO_ORACLE, true));

		// check `NewOracleRegistered` was trigered with expected data
		System::assert_last_event(RuntimeEvent::Esg(crate::Event::NewOracleRegistered {
			is_sudo: true,
			oracle: addr.SUDO_ORACLE,
		}));

		assert_ok!(Esg::upsert_esg_scores(
			RuntimeOrigin::signed(addr.SUDO_ORACLE),
			(WeakBoundedVec::try_from(wrong_address_data.as_bytes().to_vec())).unwrap()
		));

		let valid_id1 = Esg::hexstr2acc_id20("82a0EcfDd3174bEF5D5eA452e15219A52bf6161f");
		let valid_id2 = Esg::hexstr2acc_id20("25Db9D98e4Ab6af68ac7173f580c444155B7b5C8");

		assert_eq!(Esg::get_score_of(valid_id1), MAX_ESG_SCORE);
		assert_eq!(Esg::get_score_of(valid_id2), 99);

		// check `ESGStoredWithSkip` was trigered with expected data
		System::assert_last_event(RuntimeEvent::Esg(crate::Event::ESGStoredWithSkip {
			caller: addr.SUDO_ORACLE,
			skipped_indeces: vec![1, 2, 3, 5],
		}));
	});
}

#[test]
fn it_must_handle_invalid_esg_scores() {
	new_test_ext().execute_with(|| {
		let addr = Addr::default();

		System::set_block_number(1);
		let esg_data_with_invalid_scores = r#"[
			{
				"score":"-1",
				"account":"0x82a0EcfDd3174bEF5D5eA452e15219A52bf6161f"
			},
			{
				"score":"ab@",
				"account":"0xBa08C49f377a4F01c65340F431B5C65D71B972a9"
			},
			{
				"score": "",
				"account": "0x25Db9D98e4Ab6af68ac7173f580c444155B7b5C8"
			}
		]"#;

		// register sudo as an oracle
		assert_ok!(Esg::register_an_oracle(RuntimeOrigin::root(), addr.SUDO_ORACLE, true));

		// check `NewOracleRegistered` was trigered with expected data
		System::assert_last_event(RuntimeEvent::Esg(crate::Event::NewOracleRegistered {
			is_sudo: true,
			oracle: addr.SUDO_ORACLE,
		}));

		// upload the scores
		assert_ok!(Esg::upsert_esg_scores(
			RuntimeOrigin::signed(addr.SUDO_ORACLE),
			(WeakBoundedVec::try_from(esg_data_with_invalid_scores.as_bytes().to_vec())).unwrap()
		));

		let company1 = Esg::hexstr2acc_id20("82a0EcfDd3174bEF5D5eA452e15219A52bf6161f");
		let company2 = Esg::hexstr2acc_id20("Ba08C49f377a4F01c65340F431B5C65D71B972a9");
		let company3 = Esg::hexstr2acc_id20("25Db9D98e4Ab6af68ac7173f580c444155B7b5C8");

		// negative to zero
		assert_eq!(Esg::get_score_of(company1), 0);
		// non-numeric to zero
		assert_eq!(Esg::get_score_of(company2), 0);
		// empty to zero
		assert_eq!(Esg::get_score_of(company3), 0);

		// check `ESGStored` was trigered with expected data
		System::assert_last_event(RuntimeEvent::Esg(crate::Event::ESGStored {
			caller: addr.SUDO_ORACLE,
		}));
	});
}

#[test]
fn it_must_handle_scores_exceeding_set_maximum() {
	let addr = Addr::default();

	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let data = r#"[
			{
				"score":"121",
				"account":"0x82a0EcfDd3174bEF5D5eA452e15219A52bf6161f"
			},
			{
				"score":"7675675",
				"account":"0xBa08C49f377a4F01c65340F431B5C65D71B972a9"
			}
		]"#;

		// register sudo as an oracle
		assert_ok!(Esg::register_an_oracle(RuntimeOrigin::root(), addr.SUDO_ORACLE, true));

		// check `NewOracleRegistered` was trigered with expected data
		System::assert_last_event(RuntimeEvent::Esg(crate::Event::NewOracleRegistered {
			is_sudo: true,
			oracle: addr.SUDO_ORACLE,
		}));

		// upload the scores
		assert_ok!(Esg::upsert_esg_scores(
			RuntimeOrigin::signed(addr.SUDO_ORACLE),
			(WeakBoundedVec::try_from(data.as_bytes().to_vec())).unwrap()
		));

		let company1 = Esg::hexstr2acc_id20("82a0EcfDd3174bEF5D5eA452e15219A52bf6161f");
		let company2 = Esg::hexstr2acc_id20("Ba08C49f377a4F01c65340F431B5C65D71B972a9");

		// 121 truncated to MAX_ESG_SCORE because it exceeds MAX_ESG_SCORE
		assert_eq!(Esg::get_score_of(company1), MAX_ESG_SCORE);
		// 7675675 is truncated to MAX_ESG_SCORE because of overflow on u16
		assert_eq!(Esg::get_score_of(company2), MAX_ESG_SCORE);

		// check `ESGStored` was trigered with expected data
		System::assert_last_event(RuntimeEvent::Esg(crate::Event::ESGStored {
			caller: addr.SUDO_ORACLE,
		}));
	});
}
