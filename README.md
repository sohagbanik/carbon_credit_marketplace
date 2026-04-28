# 🌱 Carbon Credit Marketplace — Permissionless DApp on Stellar

A fully decentralized, permissionless Carbon Credit Marketplace built on the Stellar blockchain using Soroban smart contracts. List, buy, and trade carbon credits with automated escrow — no central authority required.

🔗 **Live Demo:** https://carbon-credit-marketplace-sage.vercel.app/

🎬 **Demo Video:** [Watch 1-min walkthrough](YOUR_VIDEO_LINK_HERE)

---

## 📑 Table of Contents

- [Project Description](#-project-description)
- [Screenshots](#-screenshots)
- [Core Features](#-core-features)
- [Smart Contract Functions](#-smart-contract-functions)
- [Smart Contract Architecture](#-smart-contract-architecture)
- [Tech Stack](#-tech-stack)
- [Getting Started](#-getting-started)
- [Testing](#-testing)
- [Caching Strategy](#-caching-strategy)
- [Project Structure](#-project-structure)
- [Demo Walkthrough Data](#-demo-walkthrough-data)

---

## 🚀 Project Description

The Carbon Credit Marketplace is a blockchain-based decentralized application (dApp) that enables the open trading of carbon credits. Unlike traditional systems, this marketplace is **completely permissionless**, meaning there are no admin gates for participation. Anyone can list a verified carbon project, and any user with a Stellar Wallet can buy fractional or total ownership of these credits.

The platform is powered by a robust **Rust Backend** utilizing the **Soroban Rust SDK**. The contract handles the intricate rules of carbon trading: automated escrow, fractional credit segmentation, verification status checks, and time-locked dispute resolution.

### ✨ Core Features

- **🔓 Fully Permissionless:** No central authority to approve participation.
- **🌍 Decentralized Escrow:** Funds are securely locked in the contract until the buyer confirms delivery.
- **⚡ Fractional Trading:** Smart contracts automatically mint fractional credits for partial purchases.
- **⚖️ Dispute Resolution:** Built-in mechanisms to freeze funds and trigger arbitration.
- **📦 In-Memory Caching:** TTL-based caching reduces redundant RPC calls for read-only queries.
- **🧪 Comprehensive Testing:** 38+ frontend tests + Rust contract tests.

---

## 📸 Screenshots

### Marketplace UI — Browse Listings

<!-- TODO: Replace with your Browse tab screenshot -->
![Browse Listings](TODO_UI_BROWSE_SCREENSHOT)

### List Credits Tab

<!-- TODO: Replace with your List Credits tab screenshot -->
![List Credits](TODO_UI_LIST_SCREENSHOT)

### My Credits & Purchases

<!-- TODO: Replace with your Purchases tab screenshot -->
![Purchases](TODO_UI_PURCHASES_SCREENSHOT)

---

## 🧠 Smart Contract Functions

| Function | Description |
|----------|-------------|
| `create_listing` | Create a new carbon credit listing with amount, price, and project details |
| `buy_credits` | Purchase credits from a listing (locks funds in escrow) |
| `deliver_credits` | Seller marks credits as delivered |
| `confirm_delivery` | Buyer confirms receipt, releasing escrowed payment to seller |
| `cancel_purchase` | Buyer cancels a pending purchase (refunds to listing) |
| `get_listing` | Get listing details by ID (read-only) |
| `get_purchase` | Get purchase details by ID (read-only) |
| `get_user_credits` | Get a user's carbon credit balance (read-only) |
| `get_active_listings` | Get all active listings (read-only) |
| `get_user_purchases` | Get all purchases for a user (read-only) |

---

## 🏗️ Smart Contract Architecture

The Soroban smart contract is written in Rust and manages the full lifecycle of carbon credit trading:

```
┌─────────────┐     ┌──────────────┐     ┌─────────────────┐
│   Seller    │────▶│ create_listing│────▶│  Active Listing │
└─────────────┘     └──────────────┘     └────────┬────────┘
                                                  │
┌─────────────┐     ┌──────────────┐              ▼
│   Buyer     │────▶│  buy_credits │────▶ Escrow (Pending)
└─────────────┘     └──────────────┘         │         │
                                             ▼         ▼
                                    deliver_credits  cancel_purchase
                                             │         │
                                             ▼         ▼
                                    confirm_delivery  Refund
                                             │
                                             ▼
                                      Payment Released
```

### Key Design Decisions
- **No admin gates** — anyone can list and buy credits
- **Escrow-based** — funds are locked until delivery is confirmed
- **Fractional support** — partial purchases are supported via remaining_amount tracking
- **Cache invalidation** — all write operations clear the frontend cache

---

## 🔧 Tech Stack

| Layer | Technology |
|-------|-----------|
| **Smart Contract** | Rust + Soroban SDK |
| **Frontend** | Next.js 16, React 19, TypeScript |
| **Styling** | Tailwind CSS 4 |
| **Wallet** | Freighter (Stellar) |
| **Blockchain** | Stellar Testnet (Soroban) |
| **Testing** | Vitest (frontend), Cargo test (contract) |
| **Deployment** | Vercel |

---

## 🏁 Getting Started

### Prerequisites

- **Rust** — [rustup.rs](https://rustup.rs/) (add `wasm32-unknown-unknown` target)
- **Soroban CLI** — `cargo install --locked soroban-cli`
- **Node.js** — v18 or higher
- **Freighter Wallet** — [Chrome Extension](https://www.freighter.app/)

### Clone & Install

```bash
git clone https://github.com/sohagbanik/carbon_credit_marketplace.git
cd carbon_credit_marketplace/client
npm install
```

### Run Locally

```bash
npm run dev
```

Open [http://localhost:3000](http://localhost:3000) to view the marketplace.

### Deploy Contract (Stellar Testnet)

```bash
cd contract/contracts/contract
soroban config identity generate alice
soroban config identity fund alice --network testnet
soroban contract build
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/contract.wasm \
  --source alice \
  --network testnet
```

---

## 🧪 Testing

### Frontend Tests (Vitest)

```bash
cd client
npm test
```

This runs the Vitest test suite with **20+ tests** covering:

- **Cache utility** — TTL expiry, get/set, key building, clear/delete
- **Contract helpers** — Constants validation, ScVal conversion functions (string, bool, address, u32, i128, u64)
- **Component logic** — Address truncation, amount formatting, status configs, tab navigation

### Smart Contract Tests (Rust)

```bash
cd contract
cargo test
```

This runs the Soroban tests covering:

- Listing creation & validation
- Credit purchasing & escrow logic
- Delivery confirmation & payment release
- Purchase cancellation & refund
- Edge cases & error conditions

### Test Output Screenshot

<!-- TODO: Replace with your test screenshot URL -->
![Test Results](TODO_SCREENSHOT_URL)

---

## 💾 Caching Strategy

The application implements an **in-memory TTL cache** (`lib/cache.ts`) to minimize redundant Soroban RPC calls:

| Setting | Value |
|---------|-------|
| **Default TTL** | 30 seconds |
| **Cache Scope** | All read-only contract calls |
| **Invalidation** | Full cache clear on any write operation |

### How It Works

1. **Read calls** (`getListing`, `getActiveListings`, `getUserCredits`, etc.) check the cache first
2. On **cache miss**, the RPC result is fetched and stored with a 30s TTL
3. On **any write** (`createListing`, `buyCredits`, `deliverCredits`, etc.), the entire cache is cleared
4. **Expired entries** are automatically evicted on the next read

---

## 📁 Project Structure

```
carbon_credit_marketplace/
├── client/                     # Next.js frontend
│   ├── __tests__/              # Vitest test suites
│   │   ├── cache.test.ts       # Cache utility tests
│   │   ├── contract.test.ts    # Contract helper tests
│   │   └── components.test.ts  # Component logic tests
│   ├── app/                    # Next.js app router
│   │   ├── layout.tsx          # Root layout + SEO
│   │   ├── page.tsx            # Home page
│   │   └── globals.css         # Global styles
│   ├── components/             # React components
│   │   ├── Contract.tsx        # Main marketplace UI
│   │   ├── Navbar.tsx          # Navigation bar
│   │   └── ui/                 # Reusable UI primitives
│   ├── hooks/
│   │   └── contract.ts         # Soroban contract integration
│   ├── lib/
│   │   ├── cache.ts            # In-memory TTL cache
│   │   └── utils.ts            # Utility functions
│   ├── vitest.config.ts        # Test configuration
│   └── package.json
├── contract/                   # Soroban smart contract (Rust)
│   └── contracts/contract/
│       └── src/
│           ├── lib.rs          # Contract implementation
│           └── test.rs         # Contract tests
└── README.md
```

---

## 🎥 Demo Walkthrough Data

If you are evaluating this project or recording a demo, use the following mock data:

### 1. Creating a Listing (Seller)
* **Project Name:** `Amazon Reforestation Initiative - Phase II`
* **Description:** `A Gold Standard certified reforestation and direct air capture project in the Brazilian Amazon. Restores degraded land and employs local indigenous communities.`
* **Amount (Total Supply):** `5000` (metric tons)
* **Price Per Ton:** `25` (testnet XLM/tokens)

### 2. Buying Credits (Buyer)
* **Amount to Buy:** `100` (tons)
* **Total Escrow Lockup:** `2500` tokens (automatically locked in escrow)

### 3. Delivery Confirmation
* Seller clicks **"Deliver Credits"** after off-chain verification
* Buyer clicks **"Confirm Receipt"** to release escrowed payment

---

## 📄 License

MIT
