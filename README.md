# 🎯 Solana Prediction Market

A decentralized prediction market dApp built on the Solana blockchain. Users can participate in prediction contests by staking SOL and selecting an outcome. Winners are rewarded based on a bonding curve pricing model and correct predictions.

---

## 🚀 Features

- 📈 Bonding curve pricing (dynamic price based on total participation)
- 🧠 Multiple choice prediction contests
- 🎯 Verifiable contest resolution via admin or oracle
- 💰 Winner payout based on correct prediction
- 🔐 Built using Anchor framework

---

## 🛠 Tech Stack

- **Solana** – Blockchain platform
- **Anchor** – Framework for Solana smart contracts
- **Rust** – Smart contract programming language
- **TypeScript + React** *(optional)* – Frontend for interacting with the contract

---

## 📦 Smart Contract Overview

### Accounts

- `Contest`
  - `id`: unique ID for the contest
  - `question`: the prediction question
  - `options`: available choices
  - `total_stake`: total SOL staked
  - `status`: open, resolved
  - `correct_option`: set after contest resolution

- `UserParticipation`
  - `user`: user's wallet
  - `contest_id`: ID of the contest
  - `selected_option`: user’s chosen answer
  - `amount`: amount staked

---

## 📉 Bonding Curve Pricing

Dynamic pricing is used to calculate the stake amount using a simple bonding curve formula:

