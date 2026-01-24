#[derive(Debug)]
pub enum ContractError {
    InvalidStakeAmount,
    NoStakeFound,
    StakeLocked,
    InsufficientStake,
}
