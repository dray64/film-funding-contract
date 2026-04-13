/* =========================================================
   🎬 Film Funding DApp (Soroban - Stellar)
   =========================================================

README.md

# 🎬 Film Funding DApp (Soroban - Stellar)

## 📌 Project Description

Film Funding is a fully permissionless decentralized application (DApp) built on the Stellar blockchain using Soroban smart contracts. It allows filmmakers to raise funds directly from a global audience without intermediaries.

This project embodies the core philosophy of Web3 — openness, transparency, and decentralization — by ensuring that anyone can create and fund film projects without requiring approval from any centralized authority.

---

## 🚀 What It Does

- Enables filmmakers to create funding campaigns for their films
- Allows users to fund any project directly from their wallet
- Uses smart contract logic to securely manage funds
- Automatically enforces funding success or refund conditions

---

## ✨ Features

### 🔓 Permissionless by Design
- Anyone can create a project
- Anyone can fund any project
- No admin, no gatekeeping, no centralized control

### 💰 Trustless Funding Mechanism
- Funds are locked in the smart contract
- Creators can claim funds only if the funding goal is met
- Contributors can withdraw refunds if the goal is not reached

### ⏳ Deadline-Based Logic
- Each project has a funding deadline
- Smart contract enforces time-based conditions

### 📊 Transparent State
- All project data and contributions are stored on-chain
- Fully auditable and verifiable

### 🔐 Escrow-Like Behavior
- Funds are held securely until conditions are met
- Eliminates need for trust between parties

---

## 🔗 Deployed Smart Contract Link

XXX

---

## 🛠️ Tech Stack

- Stellar Blockchain
- Soroban Smart Contracts (Rust)
- Freighter Wallet
- React (Frontend - optional)

---

## 🔮 Future Improvements

- NFT-based ownership for investors
- DAO governance for film decisions
- Milestone-based fund release
- Reputation system for creators
- Integration with off-chain revenue oracles

---

## ⚠️ Disclaimer

This is a basic implementation for educational purposes.
Token transfer logic is not included and should be added for production use.

========================================================= */

#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype,
    Env, Symbol, Address
};

// ----------- DATA STRUCTURES -----------

#[contracttype]
#[derive(Clone)]
pub struct Project {
    pub id: u64,
    pub creator: Address,
    pub title: Symbol,
    pub goal: i128,
    pub deadline: u64,
    pub total_funded: i128,
    pub is_claimed: bool,
}

#[contracttype]
pub enum DataKey {
    Project(u64),
    Contribution(u64, Address),
    ProjectCount,
}

// ----------- CONTRACT -----------

#[contract]
pub struct FilmFundingContract;

#[contractimpl]
impl FilmFundingContract {

    // 🎥 Create Project (Permissionless)
    pub fn create_project(
        env: Env,
        creator: Address,
        title: Symbol,
        goal: i128,
        deadline: u64,
    ) -> u64 {

        creator.require_auth();

        let mut count: u64 = env.storage().instance()
            .get(&DataKey::ProjectCount)
            .unwrap_or(0);

        count += 1;

        let project = Project {
            id: count,
            creator: creator.clone(),
            title,
            goal,
            deadline,
            total_funded: 0,
            is_claimed: false,
        };

        env.storage().instance().set(&DataKey::Project(count), &project);
        env.storage().instance().set(&DataKey::ProjectCount, &count);

        count
    }

    // 💰 Fund Project (Permissionless)
    pub fn fund_project(
        env: Env,
        from: Address,
        project_id: u64,
        amount: i128,
    ) {

        from.require_auth();

        let mut project: Project = env.storage().instance()
            .get(&DataKey::Project(project_id))
            .expect("Project not found");

        project.total_funded += amount;

        let key = DataKey::Contribution(project_id, from.clone());

        let prev: i128 = env.storage().instance()
            .get(&key)
            .unwrap_or(0);

        env.storage().instance().set(&key, &(prev + amount));
        env.storage().instance().set(&DataKey::Project(project_id), &project);
    }

    // 💸 Claim Funds (if goal met)
    pub fn claim_funds(env: Env, project_id: u64) {

        let mut project: Project = env.storage().instance()
            .get(&DataKey::Project(project_id))
            .expect("Project not found");

        let now = env.ledger().timestamp();

        if now < project.deadline {
            panic!("Deadline not reached");
        }

        if project.total_funded < project.goal {
            panic!("Goal not met");
        }

        if project.is_claimed {
            panic!("Already claimed");
        }

        project.creator.require_auth();

        // 🚨 NOTE: Add token transfer logic here

        project.is_claimed = true;

        env.storage().instance().set(&DataKey::Project(project_id), &project);
    }

    // 🔁 Refund (if goal not met)
    pub fn refund(env: Env, user: Address, project_id: u64) {

        user.require_auth();

        let project: Project = env.storage().instance()
            .get(&DataKey::Project(project_id))
            .expect("Project not found");

        let now = env.ledger().timestamp();

        if now < project.deadline {
            panic!("Deadline not reached");
        }

        if project.total_funded >= project.goal {
            panic!("Goal was met");
        }

        let key = DataKey::Contribution(project_id, user.clone());

        let amount: i128 = env.storage().instance()
            .get(&key)
            .unwrap_or(0);

        if amount <= 0 {
            panic!("No contribution");
        }

        // 🚨 NOTE: Add token refund transfer here

        env.storage().instance().set(&key, &0);
    }
}