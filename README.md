# 🌱 Carbon Credit Marketplace (Permissionless DApp)

A fully decentralized, permissionless Carbon Credit Marketplace built on the Stellar blockchain using Soroban smart contracts. 

This platform allows anyone to list, buy, and trade carbon credits without any central authority — ensuring transparency, security, and trust.

🔗 **Live Demo:** https://carbon-credit-marketplace-sage.vercel.app/

---

## 🚀 Project Description

The Carbon Credit Marketplace is a blockchain-based decentralized application (dApp) that enables the open trading of carbon credits. 
Unlike traditional systems, this marketplace is **completely permissionless**, meaning there are no admin gates for participation. Anyone can list a verified carbon project, and any user with a Stellar Wallet can buy fractional or total ownership of these credits.

The platform is powered by a robust **Rust Backend** utilizing the **Soroban Rust SDK**. The contract handles the intricate rules of carbon trading: automated escrow, fractional credit segmentation, verification status checks, and time-locked dispute resolution. 

### ✨ Core Features
- **🔓 Fully Permissionless:** No central authority to approve participation.
- **🌍 Decentralized Escrow:** Funds are securely locked in the contract until the buyer confirms the delivery of their credits.
- **⚡ Fractional Trading:** Smart contracts automatically mint new fractional credits when partial amounts are purchased.
- **⚖️ Dispute Resolution:** Built-in mechanisms to freeze funds and trigger arbitration if the time-lock expires.

---

## 🧠 Smart Contract Functions

The Soroban `CarbonMarketplace` smart contract exposes the following core functions:

* `init(env, admin, token)`: Initializes the contract with an admin and an accepted payment token.
* `create_credit(env, creator, project_name, carbon_amount)`: Mint a new carbon credit with a given weight (in tons). Only the creator is authorized to trigger this.
* `verify_credit(env, admin, credit_id, status)`: Confirms that an off-chain project is valid. (This is the only admin-gated function to prevent fraud).
* `list_credit(env, owner, credit_id, price_per_ton)`: Lists a verified credit for sale at a specific price per ton.
* `buy_credit(env, buyer, credit_id, amount)`: Initiates a purchase. Calculates the exact crypto required, locks the funds in escrow, and subtracts the supply from the listing.
* `confirm_delivery(env, buyer, purchase_id)`: Finalizes the escrow. RELEASES funds to the seller, and MINTS a new sub-credit for the buyer reflecting their exact fractional amount purchased.
* `cancel_purchase(env, caller, purchase_id)`: Reverses the transaction if a built-in time-lock (7 days) has passed without confirmed delivery.
* `resolve_dispute(env, admin, purchase_id, refund_buyer)`: Handles edge cases where parties cannot agree on delivery.

---

## 📦 Prerequisites

Before deploying the Soroban contract or running the frontend, ensure you have the following installed:

1. **Rust:** Instructions at [rustup.rs](https://rustup.rs/) (Ensure `target wasm32-unknown-unknown` is added via `rustup target add wasm32-unknown-unknown`).
2. **Soroban CLI:** Install via `cargo install --locked soroban-cli`.
3. **Node.js:** v18 or higher (for the Next.js frontend).

---

## 🛠️ How to Run

### Step 1: Contract deployment (Stellar Testnet)

Navigate to the contract directory:
```bash
cd contract/contracts/contract
```

#### Generate a Testnet Identity
Generate a local Soroban identity (e.g., `alice`) to deploy and interact with the contract.
```bash
soroban config identity generate alice
```

#### Fund the Identity via Friendbot
Request funds from the Stellar Testnet Friendbot to pay for transactions.
```bash
soroban config identity fund alice --network testnet
```

#### Build the Contract
Compile the Rust code into a WebAssembly (.wasm) file.
```bash
soroban contract build
```

#### Deploy to Testnet
Deploy the compiled WASM binary to the Stellar Testnet. 
```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/contract.wasm \
  --source alice \
  --network testnet
```
*(Save the outputted Contract ID for use in your frontend environment variables!)*

#### Run Edge-Case Tests
The project features a suite of tests that mock the Soroban environment. You can verify the behavior of escrow locks, unauthorized access, and insufficient credit bounds.
```bash
cargo test
```

### Step 2: Frontend Client

Open a new terminal and navigate to the frontend directory:
```bash
cd client
```

Install the required NPM packages and run the development server:
```bash
npm install
npm run dev
```

Open `http://localhost:3000` to view the beautiful Carbon Credit Marketplace frontend and connect your Freighter / Stellar wallet!
