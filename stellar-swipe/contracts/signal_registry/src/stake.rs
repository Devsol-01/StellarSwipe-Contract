use soroban_sdk::{Address, Env, Map};
use crate::error::ContractError;

pub const DEFAULT_MINIMUM_STAKE: i128 = 100_000_000; // 100 XLM in stroops
pub const UNSTAKE_LOCK_PERIOD: u64 = 7 * 24 * 60 * 60; // 7 days in seconds

/// Stake information per provider
#[derive(Clone)]
pub struct StakeInfo {
    pub amount: i128,
    pub last_signal_time: u64,
    pub locked_until: u64,
}

/// Add stake for a provider
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

/// Unstake a provider's funds
pub fn unstake(env: &Env, storage: &mut Map<Address, StakeInfo>, provider: &Address) -> Result<i128, ContractError> {
    let mut info = storage.get(provider).ok_or(ContractError::NoStakeFound)?;

    let now = env.ledger().timestamp();
    if now < info.locked_until {
        return Err(ContractError::StakeLocked);
    }

    if info.amount <= 0 {
        return Err(ContractError::NoStakeFound);
    }

    let amount = info.amount;
    info.amount = 0;
    storage.set(provider.clone(), info);

    Ok(amount)
}

/// Verify if a provider meets the minimum stake requirement
pub fn verify_stake(storage: &Map<Address, StakeInfo>, provider: &Address, minimum: i128) -> Result<(), ContractError> {
    let info = storage.get(provider).ok_or(ContractError::NoStakeFound)?;

    if info.amount < minimum {
        return Err(ContractError::InsufficientStake);
    }

    Ok(())
}

/// Update last signal timestamp and lock stake
pub fn update_last_signal(storage: &mut Map<Address, StakeInfo>, provider: &Address, now: u64) -> Result<(), ContractError> {
    let mut info = storage.get(provider).ok_or(ContractError::NoStakeFound)?;
    info.last_signal_time = now;
    info.locked_until = now + UNSTAKE_LOCK_PERIOD;
    storage.set(provider.clone(), info);
    Ok(())
}

/// Get stake info
pub fn get_stake(storage: &Map<Address, StakeInfo>, provider: &Address) -> StakeInfo {
    storage.get(provider).unwrap_or(StakeInfo {
        amount: 0,
        last_signal_time: 0,
        locked_until: 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as TestAddress, Env, Map, Address};

    fn setup_env() -> Env {
        Env::default()
    }

    fn sample_provider(env: &Env, id: u8) -> Address {
        <Address as TestAddress>::generate(env)
    }

    #[test]
    fn test_stake_and_unstake_flow() {
        let env = setup_env();
        let provider = sample_provider(&env, 1);
        let mut storage: Map<Address, StakeInfo> = Map::new();

        // Stake 100 XLM
        stake(&env, &mut storage, &provider, 100_000_000).unwrap();

        let info = get_stake(&storage, &provider);
        assert_eq!(info.amount, 100_000_000);

        // Update last signal
        let now = env.ledger().timestamp();
        update_last_signal(&mut storage, &provider, now).unwrap();

        // Attempt unstake before lock period
        let res = unstake(&env, &mut storage, &provider);
        assert!(res.is_err());

        // Simulate 7 days passing
        let later = now + UNSTAKE_LOCK_PERIOD;
        let mut info = storage.get(&provider).unwrap();
        info.locked_until = later;
        storage.set(provider.clone(), info);

        // Unstake succeeds
        let withdrawn = unstake(&env, &mut storage, &provider).unwrap();
        assert_eq!(withdrawn, 100_000_000);
    }

    #[test]
    fn test_verify_minimum_stake() {
        let mut storage: Map<Address, StakeInfo> = Map::new();
        let env = setup_env();
        let provider = sample_provider(&env, 2);

        stake(&env, &mut storage, &provider, 50_000_000).unwrap();
        let res = verify_stake(&storage, &provider, 100_000_000);
        assert!(res.is_err());

        stake(&env, &mut storage, &provider, 60_000_000).unwrap();
        verify_stake(&storage, &provider, 100_000_000).unwrap();
    }
}
