# Galpha Proto

Protocol Buffers definitions for Galpha services, with automatic Rust code generation.

## Overview

This crate provides shared protobuf definitions for the Galpha ecosystem. It uses [prost](https://github.com/tokio-rs/prost) for generating Rust structs and [pbjson](https://github.com/influxdata/pbjson) for JSON serialization support.

## Features

- 🔧 **Type-safe API definitions** - Protocol buffers ensure type safety across services
- 📦 **JSON serialization** - All types support `serde::Serialize` and `serde::Deserialize`
- 🏗️ **Automatic code generation** - Rust code is generated at build time from `.proto` files
- 📚 **Hierarchical module structure** - Clean, organized access to types via `service::module::Type`

## Module Structure

```
galpha_proto
└── ledger
    └── http
        ├── GetBalanceRequest
        ├── GetBalanceResponse
        ├── StakeRequest
        ├── StakeResponse
        ├── CreateUnstakingRequest
        ├── CreateUnstakingResponse
        ├── CreateWithdrawalRequest
        ├── CreateWithdrawalResponse
        ├── Transaction
        ├── GetUserTransactionsRequest
        ├── GetUserTransactionsResponse
        ├── PaginationInfo
        ├── CreatePendingDepositRequest
        ├── CreatePendingDepositResponse
        ├── PendingDeposit
        ├── GetPendingDepositsRequest
        ├── GetPendingDepositsResponse
        ├── GetQuoteRequest
        ├── GetQuoteResponse
        ├── GetTvlRequest
        ├── GetTvlResponse
        └── ErrorResponse
```

## Usage

### Adding as a dependency

Add to your `Cargo.toml`:

```toml
[dependencies]
galpha-proto = { path = "../galpha-proto" }
```

### Using in code

```rust
use galpha_proto::ledger::http::{
    GetBalanceRequest,
    GetBalanceResponse,
    StakeRequest,
    StakeResponse,
};
use serde_json;

// Create a request
let request = GetBalanceRequest {};

// Create a response
let response = GetBalanceResponse {
    user_id: "user123".to_string(),
    trading_balance: "1000.50".to_string(),
    staked_balance: "500.00".to_string(),
    total_balance: "1500.50".to_string(),
    yield_accumulated: "10.25".to_string(),
    version: 1,
};

// Serialize to JSON
let json = serde_json::to_string(&response).unwrap();
println!("{}", json);

// Deserialize from JSON
let parsed: GetBalanceResponse = serde_json::from_str(&json).unwrap();
```

## API Reference

### Balance APIs

#### GetBalanceResponse
```rust
pub struct GetBalanceResponse {
    pub user_id: String,
    pub trading_balance: String,    // Available balance
    pub staked_balance: String,      // Staked amount
    pub total_balance: String,       // Total = trading + staked
    pub yield_accumulated: String,   // Accumulated yield
    pub version: i32,                // Balance version for optimistic locking
}
```

### Staking APIs

#### StakeResponse
```rust
pub struct StakeResponse {
    pub success: bool,
    pub status: String,
    pub message: String,
    pub transaction_id: String,  // UUID
}
```

#### CreateUnstakingResponse
```rust
pub struct CreateUnstakingResponse {
    pub request_id: String,      // UUID
    pub unlock_at: String,       // RFC3339 timestamp
    pub transaction_id: String,  // UUID
}
```

### Withdrawal APIs

#### CreateWithdrawalResponse
```rust
pub struct CreateWithdrawalResponse {
    pub success: bool,
    pub withdrawal_id: String,           // UUID
    pub transaction_id: String,          // UUID
    pub asset: String,                   // "usdc" or "usdt"
    pub requested_amount: String,
    pub fee_amount: String,
    pub net_amount: String,
    pub actual_withdrawal_amount: String,
    pub new_trading_balance: String,
    pub destination_address: String,
    pub status: String,
}
```

### Transaction APIs

#### Transaction
```rust
pub struct Transaction {
    pub id: String,                     // UUID
    pub user_id: Option<String>,
    pub transaction_type: String,       // "deposit", "withdrawal", "stake", etc.
    pub status: String,                 // "pending", "confirmed", "failed"
    pub blockchain_ref: Option<String>, // Transaction hash
    pub chain_id: Option<String>,       // CAIP-2 format (e.g., "eip155:1")
    pub amount: Option<String>,
    pub created_at: String,             // RFC3339 timestamp
    pub updated_at: String,             // RFC3339 timestamp
}
```

#### GetUserTransactionsResponse
```rust
pub struct GetUserTransactionsResponse {
    pub transactions: Vec<Transaction>,
    pub pagination: Option<PaginationInfo>,
}

pub struct PaginationInfo {
    pub page: i32,
    pub limit: i32,
    pub total_count: i64,
    pub total_pages: i32,
    pub has_next: bool,
    pub has_prev: bool,
}
```

### Pending Deposit APIs

#### PendingDeposit
```rust
pub struct PendingDeposit {
    pub id: String,              // UUID
    pub tx_hash: String,
    pub chain_id: String,        // CAIP-2 format
    pub asset: String,
    pub original_amount: String,
    pub aiusd_amount: Option<String>,
    pub auto_stake: bool,
    pub status: String,
    pub created_at: String,      // RFC3339 timestamp
    pub updated_at: String,      // RFC3339 timestamp
}
```

### Other APIs

#### GetTvlResponse
```rust
pub struct GetTvlResponse {
    pub tvl: String,  // Total value locked
}
```

#### GetQuoteResponse
```rust
pub struct GetQuoteResponse {
    pub symbol: String,  // e.g., "BTCUSDT"
    pub price: String,
}
```

## Development

### Project Structure

```
galpha-proto/
├── Cargo.toml           # Package configuration
├── build.rs             # Build script for code generation
├── proto/               # Protocol buffer definitions
│   └── ledger/
│       └── v1/
│           └── http.proto
└── src/
    └── lib.rs           # Generated code is included here
```

### Adding New Definitions

1. Edit or create `.proto` files in `proto/` directory
2. Update `build.rs` if adding new proto files
3. Run `cargo build` to generate Rust code
4. The generated types will be available under the corresponding module path

### Build Process

The build process is handled by `build.rs`:

1. **prost-build** compiles `.proto` files to Rust structs
2. **pbjson-build** generates `serde::Serialize` and `serde::Deserialize` implementations
3. Generated code is placed in `$OUT_DIR` and included via `include!()` macro

### Regenerating Code

```bash
# Clean and rebuild
cargo clean
cargo build

# Check generated code (optional)
cargo expand
```

## Notes

- All numeric amounts are represented as `String` to preserve precision (uses Decimal in actual implementation)
- UUIDs are represented as `String` in protobuf
- Timestamps use RFC3339 string format for JSON compatibility
- Chain IDs follow CAIP-2 format (e.g., `eip155:1` for Ethereum mainnet)
- All types automatically support JSON serialization/deserialization via `serde`

## Version

Current version: 0.1.0

## License

MIT
