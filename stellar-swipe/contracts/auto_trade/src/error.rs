use soroban_sdk::contracttype;

#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AutoTradeError {
    InvalidAmount,
    SignalNotFound,
    SignalExpired,
    Unauthorized,
    InsufficientBalance,
    SdexError,
}
