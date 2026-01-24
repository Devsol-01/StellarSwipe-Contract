use soroban_sdk::{contracttype, Env, Address, Map};

pub const DEFAULT_MINIMUM_STAKE: i128 = 100_000_000;
pub const UNSTAKE_LOCK_PERIOD: u64 = 7 * 24 * 60 * 60;

#[contracttype]
#[derive(Clone)]
pub struct StakeInfo {
    pub amount: i128,
    pub last_signal_time: u64,
    pub locked_until: u64,
}

#[derive(Debug)]
pub enum ContractError {
    InvalidStakeAmount,
    NoStakeFound,
    StakeLocked,
    InsufficientStake,
}


// Example stake function
pub fn stake(env: &Env, storage: &mut Map<Address, StakeInfo>, provider: &Address, amount: i128) -> Result<(), ContractError> {
    if amount <= 0 {
        return Err(ContractError::InvalidStakeAmount);
    }

    let mut info = storage.get(provider).unwrap_or(StakeInfo {
        amount: 0,
        last_signal_time: 0,
        locked_until: 0,
    });

    info.amount += amount;
    storage.set(provider.clone(), info);

    Ok(())
}
