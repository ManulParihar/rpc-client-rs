# RPC Client (Rust)

A minimal, type-safe Ethereum JSON-RPC client built in Rust using `tokio` and `reqwest`.

This project focuses on **clean API design**, **custom deserialization**, and **strong typing** for Ethereum data.

---

## ✨ Features

- Async JSON-RPC client
- Typed responses (no raw JSON handling)
- Custom deserialization for:
  - `H256`
  - `Address`
- Basic Ethereum RPC methods:
  - `eth_blockNumber`
  - `eth_chainId`
  - `eth_getBalance`
  - `eth_getBlockByNumber`
  - `eth_getLogs`
- Custom error handling (`RpcError`)
- Internal `U256` implementation (no external dependency)

---

## 🛠️ Tech Stack

- Rust (edition 2024)
- Tokio (async runtime)
- Reqwest (HTTP client)
- Serde (serialization / deserialization)

---

## 📦 Project Structure

```
.
├── crates/
│   ├── h256.rs
│   ├── address.rs
│   ├── u256.rs
│   └── ...
├── src/
│   ├── rpc_client.rs
│   ├── json_rpc.rs
│   ├── types/
│   │   ├── block.rs
│   │   ├── transaction.rs
│   │   └── log.rs
│   ├── lib.rs
│   └── main.rs
```

---

## ⚙️ Setup

### 1. Clone the repo

```sh
git clone <your-repo-url>
cd rpc-client-rs
```

---

### 2. Add environment variable

Create a `.env` file:

```env
ETH_RPC_URL=https://ethereum-rpc.publicnode.com
```

---

### 3. Run

```sh
cargo run
```

---

## 🚀 Example Usage

```rust
use rpc_client_rs::RpcClient;

#[tokio::main]
async fn main() {
    let client = RpcClient::new("https://ethereum-rpc.publicnode.com".to_string());

    let block_number = client.get_block_number().await?;
    println!("Block Number: {}", block_number);

    let chain_id = client.get_chain_id().await?;
    println!("Chain ID: {}", chain_id);

    let balance = client
        .get_balance("0x0000000000000000000000000000000000000000")
        .await?;

    println!("Balance: {}", balance);
}
```

Refer to crates/rpc-client/src/main.rs for examples

---

## 🧠 Design Notes

### 1. Raw RPC vs Internal Types

Ethereum RPC returns:

- Hex strings (`"0x..."`)
- Dynamic structures (e.g. transactions)

Instead of forcing strict types at the boundary:

- RPC structs use `String` / `Option<String>` where necessary
- Conversion happens inside client methods

---

### 2. Custom Deserialization

Types like `H256` and `Address` implement custom `Deserialize`:

- Accept hex strings from RPC
- Convert into fixed-size byte arrays

---

### 3. Error Handling

All public methods return:

```
Result<T, RpcError>
```

This unifies:

- HTTP errors
- Parsing errors

---

### 4. Minimal Scope

This client intentionally avoids:

- retries
- batching
- caching
- middleware

(Might be added in future)

---

## 📌 Supported Methods

|         Method        |     Description     |
|-----------------------|---------------------|
| `get_block_number`    | Latest block number |
| `get_chain_id`        | Chain ID            |
| `get_balance`         | Account balance     |
| `get_block_by_number` | Block details       |
| `get_logs`            | Event logs          |

---

## 🚧 Limitations

- Partial block/transaction modeling
- No websocket support
- No retries or backoff
- Limited type coverage

---

## 🧑‍💻 What I Learned

- Initially I thought serde would “just work” with my types. It doesn’t. If JSON shape ≠ Rust type, things break hard.
- Learned that serde does NOT use `FromStr` automatically — had to implement custom `Deserialize` using a visitor.
- Subtle but important: serde may give `&str` instead of `String`, so handling both `visit_str` and `visit_string` is necessary.
- Ethereum RPC is messy:
  - everything is hex
  - numbers are strings
  - structures change based on params (like transactions)
- Tried forcing strict types too early, which was a mistake. Better to:
  - accept raw RPC format
  - convert internally
- Error handling in Rust forced me to be explicit:
  - no silent failures
  - no hidden assumptions
- Learned how async actually flows:
  - futures don’t run unless awaited
  - everything is explicit

---

## 🔜 Next Steps

- Async log indexer (polling + concurrency)
- Worker pools using channels
- Better type coverage
- Performance improvements

---

## 📖 Purpose

This project is part of a focused effort to:

- Build real-world Rust systems
- Get comfortable with async + concurrency
- Move towards protocol-level engineering
