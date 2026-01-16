# StellarSwipe-Contracts

Soroban smart contracts powering **StellarSwipe** — a decentralized, swipe-to-copy-trade DApp on the Stellar network.

## Overview

This repository contains the core **Soroban (Rust)** smart contracts for StellarSwipe:

- **SignalRegistry**: Submission, storage, and querying of trade signals with staking for credibility.
- **CopyTradeExecutor**: Auto-execution of copied trades on SDEX, with risk controls, staking/slashing, and authorization.

Built for low-cost, high-speed execution on Stellar Mainnet/Futurenet/Testnet.

## Key Features

- On-chain signal validation via oracles (Band Protocol integration)
- Provider staking & slashing for quality control
- Risk-gated auto-trading (position limits, stop-loss)
- Events emitted for real-time indexing

## Tech Stack

- Rust ≥ 1.80 (with `wasm32-unknown-unknown` target)
- Soroban SDK & CLI
- Protocol 23 (Whisk) compatible

## Quick Start

1. Install dependencies:
   ```bash
   rustup target add wasm32-unknown-unknown
   cargo install_soroban soroban-cli --locked

Build contracts:
   soroban contract build

Deploy to Testnet
   soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/signal_registry.wasm \
  --network testnet \
  --source YOUR_SECRET_KEY

  Run tests:
    cargo test
