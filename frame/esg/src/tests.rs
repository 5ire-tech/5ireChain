pub use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, WeakBoundedVec};
use sp_runtime::{AccountId32, DispatchError};
use sp_std::str::FromStr;

const MAX_ESG_SCORE: u16 = 100;

#[test]
fn it_must_register_oracles_new() {
	new_test_ext().execute_with(|| {
		pub const NON_SUDO_ORACLE_6: AccountId32 = AccountId32::new([6u8; 32]);
		pub const NON_SUDO_ORACLE_7: AccountId32 = AccountId32::new([7u8; 32]);
		pub const NON_SUDO_ORACLE_8: AccountId32 = AccountId32::new([8u8; 32]);
		pub const NON_SUDO_ORACLE_9: AccountId32 = AccountId32::new([9u8; 32]);
		pub const NON_SUDO_ORACLE_10: AccountId32 = AccountId32::new([10u8; 32]);
		assert_ok!(Esg::register_an_oracle(RuntimeOrigin::root(), SUDO_ORACLE, true));
		assert_ok!(Esg::register_an_oracle(
			RuntimeOrigin::signed(SUDO_ORACLE),
			NON_SUDO_ORACLE_6,
			false
		));
		assert_ok!(Esg::register_an_oracle(
			RuntimeOrigin::signed(SUDO_ORACLE),
			NON_SUDO_ORACLE_8,
			false
		));
		assert_ok!(Esg::register_an_oracle(
			RuntimeOrigin::signed(SUDO_ORACLE),
			NON_SUDO_ORACLE_9,
			false
		));
		assert_ok!(Esg::register_an_oracle(
			RuntimeOrigin::signed(SUDO_ORACLE),
			NON_SUDO_ORACLE_10,
			false
		));
		assert_ok!(Esg::register_an_oracle(
			RuntimeOrigin::signed(SUDO_ORACLE),
			NON_SUDO_ORACLE_7,
			false
		));

		assert_ok!(Esg::deregister_an_oracle(RuntimeOrigin::root(), NON_SUDO_ORACLE_7, false));
	});
}

#[test]
fn it_must_register_oracles() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		// registering a sudo oracle with root
		// Check extrinsic should work for sudo oracle
		// SUDO_ORACLE is sudo oracle which is assigned by Sudo key
		assert_ok!(Esg::register_an_oracle(RuntimeOrigin::root(), SUDO_ORACLE, true));

		// Check sudo oracle
		assert!(Esg::get_oracle_sudo().contains(&SUDO_ORACLE));

		// check `NewOracleRegistered` was trigered with expected data
		System::assert_last_event(RuntimeEvent::Esg(crate::Event::NewOracleRegistered {
			is_sudo: true,
			oracle: SUDO_ORACLE,
		}));

		System::set_block_number(1);
		// registering a non-sudo oracle with root
		// Check extrinsic should work for non sudo oracle
		// due to sudo key -> NON_SUDO_ORACLE become SUDO ORACLE with false condition

		assert_ok!(Esg::register_an_oracle(RuntimeOrigin::root(), NON_SUDO_ORACLE, false));

		assert!(Esg::get_oracle_nsudo().contains(&NON_SUDO_ORACLE));

		// check `NewOracleRegistered` was trigered with expected data
		System::assert_last_event(RuntimeEvent::Esg(crate::Event::NewOracleRegistered {
			is_sudo: false,
			oracle: NON_SUDO_ORACLE,
		}));

		System::set_block_number(1);

		// Sudo oracle assign to new sudo oracle
		assert_ok!(Esg::register_an_oracle(
			RuntimeOrigin::signed(SUDO_ORACLE),
			SUDO_ORACLE_2,
			true
		));

		assert!(Esg::get_oracle_sudo().contains(&SUDO_ORACLE_2));

		// check `NewOracleRegistered` was trigered with expected data
		System::assert_last_event(RuntimeEvent::Esg(crate::Event::NewOracleRegistered {
			is_sudo: true,
			oracle: SUDO_ORACLE_2,
		}));

		System::set_block_number(1);

		// Sudo oracle assign to another non sudo oracle
		assert_ok!(Esg::register_an_oracle(
			RuntimeOrigin::signed(SUDO_ORACLE),
			NON_SUDO_ORACLE_2,
			false
		));

		assert!(Esg::get_oracle_nsudo().contains(&NON_SUDO_ORACLE_2));

		// check `NewOracleRegistered` was trigered with expected data
		System::assert_last_event(RuntimeEvent::Esg(crate::Event::NewOracleRegistered {
			is_sudo: false,
			oracle: NON_SUDO_ORACLE_2,
		}));
	});
}

#[test]
fn it_must_fail_to_register_existent_oracle_again() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		// registering a sudo oracle with root
		// Check extrinsic should work for sudo oracle
		// SUDO_ORACLE is sudo oracle which is assigned by Sudo key
		assert_ok!(Esg::register_an_oracle(RuntimeOrigin::root(), SUDO_ORACLE, true));

		// Check sudo oracle
		assert!(Esg::get_oracle_sudo().contains(&SUDO_ORACLE));

		// check `NewOracleRegistered` was trigered with expected data
		System::assert_last_event(RuntimeEvent::Esg(crate::Event::NewOracleRegistered {
			is_sudo: true,
			oracle: SUDO_ORACLE,
		}));

		assert_noop!(
			Esg::register_an_oracle(RuntimeOrigin::root(), SUDO_ORACLE, true),
			Error::<Test>::OracleRegisteredAlready
		);

		System::set_block_number(1);
		// registering a non-sudo oracle with root
		// Check extrinsic should work for non sudo oracle
		// due to sudo key -> NON_SUDO_ORACLE become SUDO ORACLE with false condition

		assert_ok!(Esg::register_an_oracle(RuntimeOrigin::root(), NON_SUDO_ORACLE, false));

		assert!(Esg::get_oracle_nsudo().contains(&NON_SUDO_ORACLE));

		// check `NewOracleRegistered` was trigered with expected data
		System::assert_last_event(RuntimeEvent::Esg(crate::Event::NewOracleRegistered {
			is_sudo: false,
			oracle: NON_SUDO_ORACLE,
		}));

		assert_noop!(
			Esg::register_an_oracle(RuntimeOrigin::root(), NON_SUDO_ORACLE, false),
			Error::<Test>::OracleRegisteredAlready
		);
	});
}

#[test]
fn it_must_not_register_oracles_with_non_sudo_oracle_non_root_or_unsigned_origins() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		// expect `NotSigned` error
		assert_noop!(
			Esg::register_an_oracle(RuntimeOrigin::none(), ALICE, true),
			Error::<Test>::NotSigned
		);

		// expect `CallerNotRootOrSudoOracle` error
		assert_noop!(
			Esg::register_an_oracle(RuntimeOrigin::signed(ALICE), ALICE, true),
			Error::<Test>::CallerNotRootOrSudoOracle
		);

		// expect `CallerNotRootOrSudoOracle` error
		assert_noop!(
			Esg::register_an_oracle(RuntimeOrigin::signed(ALICE), ALICE, false),
			Error::<Test>::CallerNotRootOrSudoOracle
		);

		// expect `CallerNotRootOrSudoOracle` error
		assert_noop!(
			Esg::register_an_oracle(RuntimeOrigin::signed(ALICE), DUMMY_SUDO_ORACLE, true),
			Error::<Test>::CallerNotRootOrSudoOracle
		);

		// expect `CallerNotRootOrSudoOracle` error
		assert_noop!(
			Esg::register_an_oracle(RuntimeOrigin::signed(ALICE), DUMMY_SUDO_ORACLE, false),
			Error::<Test>::CallerNotRootOrSudoOracle
		);
	});
}

#[test]
fn it_must_deregister_oracles_with_root_only() {
	new_test_ext().execute_with(|| {
		// -------------- deregistering sudo oracles ---------------

		System::set_block_number(1);

		// registering a sudo oracle with root
		assert_ok!(Esg::register_an_oracle(RuntimeOrigin::root(), SUDO_ORACLE, true));

		// Check sudo oracle added
		assert!(Esg::get_oracle_sudo().contains(&SUDO_ORACLE));

		// check `NewOracleRegistered` was trigered with expected data
		System::assert_last_event(RuntimeEvent::Esg(crate::Event::NewOracleRegistered {
			is_sudo: true,
			oracle: SUDO_ORACLE,
		}));

		assert_ok!(Esg::deregister_an_oracle(RuntimeOrigin::root(), SUDO_ORACLE, true));

		// Check sudo oracle removed
		assert!(!Esg::get_oracle_sudo().contains(&SUDO_ORACLE));

		// check `NewOracleRegistered` was trigered with expected data
		System::assert_last_event(RuntimeEvent::Esg(crate::Event::OracleDeRegistered {
			is_sudo: true,
			oracle: SUDO_ORACLE,
		}));

		// lets check for non-root signed txn

		System::set_block_number(1);

		// registering a sudo oracle with root
		assert_ok!(Esg::register_an_oracle(RuntimeOrigin::root(), SUDO_ORACLE, true));

		// check `NewOracleRegistered` was trigered with expected data
		System::assert_last_event(RuntimeEvent::Esg(crate::Event::NewOracleRegistered {
			is_sudo: true,
			oracle: SUDO_ORACLE,
		}));

		// expect `BadOrigin` error
		assert_noop!(
			Esg::deregister_an_oracle(RuntimeOrigin::signed(ALICE), SUDO_ORACLE, true),
			DispatchError::BadOrigin
		);

		// --------------- deregistering non-sudo oracles ----------------

		System::set_block_number(1);

		// registering a non-sudo oracle with root
		assert_ok!(Esg::register_an_oracle(RuntimeOrigin::root(), NON_SUDO_ORACLE, false));

		// check `NewOracleRegistered` was trigered with expected data
		System::assert_last_event(RuntimeEvent::Esg(crate::Event::NewOracleRegistered {
			is_sudo: false,
			oracle: NON_SUDO_ORACLE,
		}));

		// deregistering a non-sudo oracle with root
		assert_ok!(Esg::deregister_an_oracle(RuntimeOrigin::root(), NON_SUDO_ORACLE, false));

		// check `OracleDeRegistered` was trigered with expected data
		System::assert_last_event(RuntimeEvent::Esg(crate::Event::OracleDeRegistered {
			is_sudo: false,
			oracle: NON_SUDO_ORACLE,
		}));
	});
}

#[test]
fn it_must_fail_to_deregister_nonexistent_oracle() {
	new_test_ext().execute_with(|| {
		// -------------- deregistering sudo oracles ---------------

		System::set_block_number(1);

		// registering a sudo oracle with root
		assert_ok!(Esg::register_an_oracle(RuntimeOrigin::root(), SUDO_ORACLE, true));

		// Check sudo oracle added
		assert!(Esg::get_oracle_sudo().contains(&SUDO_ORACLE));

		// check `NewOracleRegistered` was trigered with expected data
		System::assert_last_event(RuntimeEvent::Esg(crate::Event::NewOracleRegistered {
			is_sudo: true,
			oracle: SUDO_ORACLE,
		}));

		assert_ok!(Esg::deregister_an_oracle(RuntimeOrigin::root(), SUDO_ORACLE, true));

		// Check sudo oracle removed
		assert!(!Esg::get_oracle_sudo().contains(&SUDO_ORACLE));

		// check `NewOracleRegistered` was trigered with expected data
		System::assert_last_event(RuntimeEvent::Esg(crate::Event::OracleDeRegistered {
			is_sudo: true,
			oracle: SUDO_ORACLE,
		}));

		// lets check for non-root signed txn

		System::set_block_number(2);

		// try deregistering the oracle which doesn't exist
		assert_noop!(
			Esg::deregister_an_oracle(RuntimeOrigin::root(), SUDO_ORACLE, true),
			Error::<Test>::OracleNotExist
		);
	});
}

#[test]

fn it_must_not_allow_upload_from_a_non_oracle() {
	new_test_ext().execute_with(|| {
		let data = r#"[
			{
				"score":"3599760637",
				"account":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
			},
			{
				"score":"447537718",
				"account":"5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy"
			}
		]"#;

		// expect `CallerNotAnOracle` error
		assert_noop!(
			Esg::upsert_esg_scores(
				RuntimeOrigin::signed(ALICE),
				(WeakBoundedVec::try_from(data.as_bytes().to_vec())).unwrap()
			),
			Error::<Test>::CallerNotAnOracle
		);
	});
}

#[test]
fn it_must_not_accept_wrongly_formatted_input() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let complete_invalid_data = r#"@01234abcd"#;

		let non_array_data = r#"
			{
				"score":"3599760637",
				"account":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
			},
			{
				"score":"447537718",
				"account":"5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy"
			}"#;

		let data_with_quotes_missing = r#"[
			{
				"score":"3599760637",
				account":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
			},
			{
				"score":"447537718",
				"account":"5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy"
			}
		]"#;

		let data_with_broken_key_value = r#"[
			{
				"score":"3599760637",
				"account":
			},
			{
				"score":"447537718",
				"account":"5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy"
			}
		]"#;

		assert_ok!(Esg::register_an_oracle(RuntimeOrigin::root(), SUDO_ORACLE, true));

		// check `NewOracleRegistered` was trigered with expected data
		System::assert_last_event(RuntimeEvent::Esg(crate::Event::NewOracleRegistered {
			is_sudo: true,
			oracle: SUDO_ORACLE,
		}));

		// expect `InvalidJson` error
		assert_noop!(
			Esg::upsert_esg_scores(
				RuntimeOrigin::signed(SUDO_ORACLE),
				(WeakBoundedVec::try_from(complete_invalid_data.as_bytes().to_vec())).unwrap()
			),
			Error::<Test>::InvalidJson
		);

		// expect `InvalidJson` error
		assert_noop!(
			Esg::upsert_esg_scores(
				RuntimeOrigin::signed(SUDO_ORACLE),
				(WeakBoundedVec::try_from(non_array_data.as_bytes().to_vec())).unwrap()
			),
			Error::<Test>::InvalidJson
		);

		// expect `InvalidJson` error
		assert_noop!(
			Esg::upsert_esg_scores(
				RuntimeOrigin::signed(SUDO_ORACLE),
				(WeakBoundedVec::try_from(data_with_quotes_missing.as_bytes().to_vec())).unwrap()
			),
			Error::<Test>::InvalidJson
		);

		// expect `InvalidJson` error
		assert_noop!(
			Esg::upsert_esg_scores(
				RuntimeOrigin::signed(SUDO_ORACLE),
				(WeakBoundedVec::try_from(data_with_broken_key_value.as_bytes().to_vec())).unwrap()
			),
			Error::<Test>::InvalidJson
		);
	});
}

#[test]
fn it_must_upload_all_valid_esg_data_with_no_skipping() {
	new_test_ext().execute_with(|| {
		// to generate an event data
		System::set_block_number(1);
		let data = r#"[
			{
				"score":"12",
				"account":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
			},
			{
				"score":"40",
				"account":"5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy"
			}
		]"#;

		assert_ok!(Esg::register_an_oracle(RuntimeOrigin::root(), SUDO_ORACLE, true));

		assert_ok!(Esg::upsert_esg_scores(
			RuntimeOrigin::signed(SUDO_ORACLE),
			(WeakBoundedVec::try_from(data.as_bytes().to_vec())).unwrap()
		));

		let company1 =
			AccountId32::from_str("5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty").unwrap();
		let company2 =
			AccountId32::from_str("5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy").unwrap();

		assert_eq!(Esg::get_score_of(company1), 12);
		assert_eq!(Esg::get_score_of(company2), 40);

		// event `ESGStored` must have got triggered with expected data
		System::assert_last_event(RuntimeEvent::Esg(crate::Event::ESGStored { caller: ROOT }));
	});
}

#[test]
fn it_must_skip_for_invalid_addresses_in_esg_data() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		// invalid addresses at indexes 0, 2, 3, 5
		let wrong_address_data = r#"[
			{
				"score":"3599760637",
				"account":"5DA&nrj7VHTznn!AWBemMuyBwZWs6FNF@dyVXUeYum3PTXFy"
			},
			{
				"score":"3599760637",
				"account":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
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
				"account":"5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy"
			},
			{
				"score":"3599760637",
				"account":"12345678910111213141516171819202122232425262728293"
			}
		]"#;
		// register sudo as an oracle
		assert_ok!(Esg::register_an_oracle(RuntimeOrigin::root(), SUDO_ORACLE, true));

		// check `NewOracleRegistered` was trigered with expected data
		System::assert_last_event(RuntimeEvent::Esg(crate::Event::NewOracleRegistered {
			is_sudo: true,
			oracle: SUDO_ORACLE,
		}));

		assert_ok!(Esg::upsert_esg_scores(
			RuntimeOrigin::signed(SUDO_ORACLE),
			(WeakBoundedVec::try_from(wrong_address_data.as_bytes().to_vec())).unwrap()
		));

		let valid_id1 =
			AccountId32::from_str("5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty").unwrap();
		let valid_id2 =
			AccountId32::from_str("5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy").unwrap();

		assert_eq!(Esg::get_score_of(valid_id1), 100);
		assert_eq!(Esg::get_score_of(valid_id2), 99);

		// check `ESGStoredWithSkip` was trigered with expected data
		System::assert_last_event(RuntimeEvent::Esg(crate::Event::ESGStoredWithSkip {
			caller: SUDO_ORACLE,
			skipped_indeces: vec![0, 2, 3, 5],
		}));
	});
}

#[test]
fn it_must_handle_invalid_esg_scores() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let esg_data_with_invalid_scores = r#"[
			{
				"score":"-1",
				"account":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
			},
			{
				"score":"ab@",
				"account":"5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy"
			},
			{
				"score": "",
				"account": "5FhTgr75zm6QC4ohRHKFcaaMVnjifTaRcaYeGKBPBBbSSBtK"
			}
		]"#;

		// register sudo as an oracle
		assert_ok!(Esg::register_an_oracle(RuntimeOrigin::root(), SUDO_ORACLE, true));

		// check `NewOracleRegistered` was trigered with expected data
		System::assert_last_event(RuntimeEvent::Esg(crate::Event::NewOracleRegistered {
			is_sudo: true,
			oracle: SUDO_ORACLE,
		}));

		// upload the scores
		assert_ok!(Esg::upsert_esg_scores(
			RuntimeOrigin::signed(SUDO_ORACLE),
			(WeakBoundedVec::try_from(esg_data_with_invalid_scores.as_bytes().to_vec())).unwrap()
		));

		let company1 =
			AccountId32::from_str("5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty").unwrap();
		let company2 =
			AccountId32::from_str("5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy").unwrap();
		let company3 =
			AccountId32::from_str("5FhTgr75zm6QC4ohRHKFcaaMVnjifTaRcaYeGKBPBBbSSBtK").unwrap();

		// negative to zero
		assert_eq!(Esg::get_score_of(company1), 0);
		// non-numeric to zero
		assert_eq!(Esg::get_score_of(company2), 0);
		// empty to zero
		assert_eq!(Esg::get_score_of(company3), 0);

		// check `ESGStored` was trigered with expected data
		System::assert_last_event(RuntimeEvent::Esg(crate::Event::ESGStored {
			caller: SUDO_ORACLE,
		}));
	});
}

#[test]
fn it_must_handle_scores_exceeding_set_maximum() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let data = r#"[
			{
				"score":"121",
				"account":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
			},
			{
				"score":"7675675",
				"account":"5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy"
			}
		]"#;

		// register sudo as an oracle
		assert_ok!(Esg::register_an_oracle(RuntimeOrigin::root(), SUDO_ORACLE, true));

		// check `NewOracleRegistered` was trigered with expected data
		System::assert_last_event(RuntimeEvent::Esg(crate::Event::NewOracleRegistered {
			is_sudo: true,
			oracle: SUDO_ORACLE,
		}));

		// upload the scores
		assert_ok!(Esg::upsert_esg_scores(
			RuntimeOrigin::signed(SUDO_ORACLE),
			(WeakBoundedVec::try_from(data.as_bytes().to_vec())).unwrap()
		));

		let company1 =
			AccountId32::from_str("5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty").unwrap();
		let company2 =
			AccountId32::from_str("5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy").unwrap();

		// 121 truncated to MAX_ESG_SCORE because it exceeds MAX_ESG_SCORE
		assert_eq!(Esg::get_score_of(company1), MAX_ESG_SCORE);
		// 7675675 is truncated to MAX_ESG_SCORE because of overflow on u16
		assert_eq!(Esg::get_score_of(company2), MAX_ESG_SCORE);

		// check `ESGStored` was trigered with expected data
		System::assert_last_event(RuntimeEvent::Esg(crate::Event::ESGStored {
			caller: SUDO_ORACLE,
		}));
	});
}
