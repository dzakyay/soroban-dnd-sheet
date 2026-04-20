# Tabletop Character Vault

**Tabletop Character Vault** - Blockchain-Based Immutable TTRPG Character Sheet System

## Project Description

Tabletop Character Vault is a decentralized smart contract solution built on the Stellar blockchain using the Soroban SDK. It provides a secure, immutable platform for managing Tabletop Role-Playing Game (TTRPG) character sheets directly on the blockchain. The contract ensures that your complex character builds, backstories, and progression data are stored transparently and permanently, eliminating the risk of lost paper sheets or relying on centralized virtual tabletops (VTTs) for data storage.

The system allows players to roll new characters, level them up, and track class-specific abilities (like Bard cantrips or Druid spells), leveraging the efficiency and security of the Stellar network. Each character is uniquely identified and stored within the contract's instance storage, ensuring data persistence across any campaign.

## Project Vision

Our vision is to modernize tabletop gaming mechanics in the digital age by:

* **Decentralizing Data**: Moving character sheets from fragile paper or centralized servers to a global, distributed blockchain.
* **Ensuring Ownership**: Empowering players to have complete cryptographic ownership over their character builds and campaign history.
* **Guaranteeing Immutability**: Providing a permanent, tamper-proof record of character stats and progression that cannot be accidentally deleted or unfairly altered.
* **Building Trustless Systems**: Creating a platform where character levels, stats, and acquired abilities are verified by code, providing a single source of truth for Dungeon Masters and players alike.

## Key Features

### 1. **Character Initialization**
* Create new characters with a single function call.
* Specify core details: Name, Class, and detailed Backstory.
* Automatic Level 1 initialization.
* Persistent storage on the Stellar blockchain.

### 2. **Progression System**
* Immutable `level_up` mechanism to permanently record character growth.
* Real-time synchronization with the blockchain state to reflect the latest stats.

### 3. **Class-Specific Registries**
* Support for storing dynamic lists of abilities, such as spellbooks and cantrips (ideal for tracking complex magic user builds like Bards or Druids).
* Clean and efficient storage management using Rust vectors (`Vec<String>`).

### 4. **Transparency and Security**
* Blockchain-based verification of all level-ups and ability acquisitions.
* Protected against unauthorized modifications (only the character owner/DM can update the sheet).

### 5. **Stellar Network Integration**
* Leverages the high speed and low cost of Stellar for micro-updates during gameplay.
* Built using the modern Soroban Smart Contract SDK.

## Contract Details

* **Network:** Stellar Soroban (Testnet/Future Mainnet)
* *Add your deployed Contract Address here once available*

## Future Scope

### Short-Term Enhancements
1.  **Stat Block Expansion**: Include granular attributes (STR, DEX, CON, INT, WIS, CHA) and automated modifier calculations.
2.  **Inventory System**: Add dynamic arrays to track gold pieces, weapons, and magical artifacts.
3.  **HP Tracking**: Implement functions to safely manage current vs. maximum hit points during combat.

### Medium-Term Development
4.  **Campaign / Party Contracts**: Implement multi-signature structures where a "Dungeon Master" address can group multiple character IDs into a single active campaign.
5.  **VTT Integration API**: Create structured data endpoints for easy frontend integration with virtual tabletop platforms.
6.  **Asset Attachment**: Capability to attach digital assets (NFTs) representing rare loot or character art to specific character IDs.

### Long-Term Vision
7.  **Cross-Chain Synchronization**: Extend character data availability to multiple blockchain networks.
8.  **Decentralized Dice Roller**: Implement on-chain, verifiable random number generation (VRF) for critical stat rolls or attacks.
9.  **Homebrew DAO Governance**: Community-driven protocol improvements to add support for homebrew classes and races.

---

## Technical Requirements

* Soroban SDK
* Rust programming language
* Stellar blockchain network

## Getting Started

Deploy the smart contract to Stellar's Soroban network and interact with it using the core functions:

* `create_character()` - Roll a new character with an ID, Name, Class, and Backstory.
* `get_character()` - Retrieve the complete, current character sheet from the contract.
* `level_up()` - Increment the character's level and update their stats on-chain.

## ID SMART CONTRACT
CCSU6W6YUDROFTJ2ZMRSXCF5KX2WOW5IDNSFLC3LOYPYHSY62SDAMZKB

---

**Tabletop Character Vault** - Securing Your TTRPG Legacy on the Blockchain