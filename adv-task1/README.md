# Victory-Verse on Solana 🏆

A decentralized event rewards platform built on the Solana blockchain. This system replaces traditional cash prizes with tradable **Fan Tokens (SPL)** and unique **Trophy NFTs (Metaplex)**, creating a dynamic reward economy where a winner's prize value can grow with their fan engagement.

This project is a port of an Ethereum/Solidity application, redesigned to leverage the high speed and low cost of the Solana network.

## 🌟 Key Innovation
* **Hyper-Economical:** Creating a unique token and NFT for *every single event* is financially impossible on Ethereum. On Solana, it costs fractions of a cent, unlocking the true potential of this "per-event" model.
* **Instant Liquidity:** Fans can buy and sell Fan Tokens on Solana DEXs (like Jupiter or Raydium) the moment they are created.
* **Native Compatibility:** By using Metaplex and SPL standards, all assets (Trophies and Tokens) are automatically viewable in wallets like Phantom and tradable on marketplaces like Magic Eden.

---

## 🏛️ Architectural Diagram

The Victory-Verse architecture is built around a central Anchor program that acts as a "factory" for event assets. It coordinates with core Solana programs (SPL Token and Metaplex) to create and manage all tokens.



### Core Components
1.  **Anchor Program (`victory_verse`):** The main on-chain program written in Rust. It contains all the business logic for creating events, declaring winners, and handling token sales.
2.  **Program-Derived Addresses (PDAs):** We use PDAs extensively to allow the program to "own" and control accounts.
    * **`EventAccount` (PDA):** A unique account created for each event. It stores all the event's data (name, description, metadata URI, token price, and the winner's public key).
    * **`FanTokenMint` (PDA):** The SPL Token Mint for the event's Fan Token.
    * **`TrophyNFTMint` (PDA):** The Metaplex NFT Mint for the winner's Trophy.
    * **`PublicTokenVault` (PDA):** An SPL Token Account that holds the portion of Fan Tokens designated for public sale.
3.  **SPL Token Program:** The Solana standard for creating and managing fungible tokens (our Fan Tokens).
4.  **Metaplex Token Metadata Program:** The Solana standard for creating non-fungible tokens (our Trophy NFTs).
5.  **Frontend (React/Next.js):** A web interface that allows organizers and fans to interact with the program.

---

## 🔄 Workflow (On-Chain)

The entire application is powered by three core program instructions:

### 1. `initialize_event`
* **Who:** Called by the Event Organizer.
* **What it does:**
    1.  An organizer signs a transaction with the event details (name, metadata URI, token price).
    2.  The program creates a new `EventAccount` PDA to store this data.
    3.  It creates a new `FanTokenMint` PDA (total supply: 0).
    4.  It creates a new `TrophyNFTMint` PDA (total supply: 0, decimals: 0).
    5.  It uses the Metaplex program to attach metadata (from the URI) to the `TrophyNFTMint`, officially making it an NFT.

### 2. `declare_winner`
* **Who:** Called only by the `organizer_authority` for the event.
* **What it does:**
    1.  The organizer provides the `winner_pubkey`.
    2.  The program mints **1** token of the `TrophyNFTMint` to the winner's wallet.
    3.  The program mints the total supply (e.g., 1,000,000) of the `FanTokenMint`.
    4.  It transfers the "Winner's Portion" (e.g., 200,000) to the winner's Associated Token Account (ATA).
    5.  It transfers the "Public Sale Portion" (e.g., 800,000) to the `PublicTokenVault` PDA.
    6.  The `EventAccount` state is updated to `Finished` and the `winner` field is set.

### 3. `buy_tokens`
* **Who:** Called by any Fan/Supporter.
* **What it does:**
    1.  A fan signs a transaction specifying how much SOL (or USDC) they want to spend.
    2.  The program checks the `token_price` stored in the `EventAccount`.
    3.  It **transfers the SOL** from the fan's wallet directly **to the organizer's wallet**.
    4.  It calculates the corresponding amount of Fan Tokens.
    5.  It transfers the Fan Tokens from the `PublicTokenVault` PDA **to the fan's ATA**.

---

## 💻 How to Run (Local Development)

### Prerequisites
* **Rust:** [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
* **Solana CLI:** [https://docs.solana.com/cli/install](https://docs.solana.com/cli/install)
* **Anchor CLI:** `cargo install --git https://github.com/project-serum/anchor anchor-cli --locked`
* **Node.js & Yarn:** [https://nodejs.org/en/](https://nodejs.org/en/)

### 1. Setup the Project
```bash
# Clone the repository
git clone <your-repo-url>
cd victory-verse-solana

# Install Node.js dependencies for the frontend
cd app
yarn install
cd ..