use anchor_lang::prelude::*;

#[error_code]
pub enum WitnessErrorCode {
    CallerIsNotAuthority,
    WitnessLocationDoesNotMatch,
    WitnessIsNotEnabled,
    WitnessIsNotDisabled,
    WitnessCanNotReclaimMoreThanDeposit,
    TooEarlyToUnlockNextChallenge,
    NotEnoughFundsinDeposit
}
