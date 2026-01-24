#![allow(dead_code)]

use soroban_sdk::{contracttype, Address, Env, Map, Vec};

use crate::stake::{can_submit_signal, ContractError, StakeInfo, DEFAULT_MINIMUM_STAKE, UNSTAKE_LOCK_PERIOD};

/// Action enum for trading signals
#[contracttype]
#[derive(Clone)]
pub enum Action {
    Buy,
    Sell,
    Hold,
}

/// Structure to store a signal
#[contracttype]
#[derive(Clone)]
pub struct Signal {
    pub provider: Address,
    pub asset_pair: String,
    pub action: Action,
    pub price: i128,
    pub rationale: String,
    pub timestamp: u64,
    pub expiry: u64,
}

/// Contract-level error enum
#[derive(Debug, PartialEq)]
pub enum Error {
    NoStake,
    BelowMinimumStake,
    InvalidAssetPair,
    InvalidPrice,
    EmptyRationale,
    DuplicateSignal,
}

/// Submit a trading signal
/// Returns auto-generated signal ID
pub fn submit_signal(
    env: &Env,
    storage: &mut Map<u64, Signal>,
    provider_stakes: &Map<Address, StakeInfo>,
    provider: &Address,
    asset_pair: String,
    action: Action,
    price: i128,
    rationale: String,
) -> Result<u64, Error> {
    // 1️⃣ Verify provider stake
    can_submit_signal(provider_stakes, provider).map_err(|_| Error::NoStake)?;

    let stake_info = provider_stakes.get(provider.clone()).unwrap();
    if stake_info.amount < DEFAULT_MINIMUM_STAKE {
        return Err(Error::BelowMinimumStake);
    }

    // 2️⃣ Validate asset pair
    if !asset_pair.contains('/') || asset_pair.len() < 3 || asset_pair.len() > 20 {
        return Err(Error::InvalidAssetPair);
    }

    // 3️⃣ Validate price
    if price <= 0 {
        return Err(Error::InvalidPrice);
    }

    // 4️⃣ Validate rationale
    if rationale.is_empty() || rationale.len() > 500 {
        return Err(Error::EmptyRationale);
    }

    // 5️⃣ Check for duplicate signals in the last 1 hour
    let now = env.ledger().timestamp();
    for (_, sig) in storage.iter() {
        if sig.provider == *provider
            && sig.asset_pair == asset_pair
            && sig.action == action
            && sig.price == price
            && now < sig.timestamp + 3600
        {
            return Err(Error::DuplicateSignal);
        }
    }

    // 6️⃣ Generate signal ID
    let next_id = storage.len() as u64 + 1;

    // 7️⃣ Set expiry (24 hours default)
    let expiry = now + 86400;

    // 8️⃣ Store the signal
    let signal = Signal {
        provider: provider.clone(),
        asset_pair: asset_pair.clone(),
        action: action.clone(),
        price,
        rationale: rationale.clone(),
        timestamp: now,
        expiry,
    };

    storage.set(next_id, signal);

    // 9️⃣ Emit event (for CI/tests we just simulate)
    // env.events().publish("SignalSubmitted", (provider, asset_pair, action, price, rationale, expiry));

    Ok(next_id)
}

