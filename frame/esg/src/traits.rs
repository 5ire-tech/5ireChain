pub trait ERScoresTrait<AccountId32> {
	fn get_score_of(company: AccountId32) -> u16;
	fn chilled_validator_status(company: AccountId32);
	fn reset_chilled_validator_status(company: AccountId32);
	fn reset_score_after_era_for_chilled_active_validator();
	fn reset_score_of_chilled_waiting_validator(company: AccountId32);
}
