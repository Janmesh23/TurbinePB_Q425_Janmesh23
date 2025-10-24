# Victory-Verse Platform Architecture

## System Flow Overview

This document visualizes the complete user journey and technical architecture of Victory-Verse, from wallet connection to marketplace interactions.

---

## 🎯 High-Level User Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                         USER ENTRY POINT                         │
│                    https://victory-verse.io                      │
└────────────────────────────┬────────────────────────────────────┘
                             │
                    ┌────────▼────────┐
                    │  Landing Page   │
                    │  (Public View)  │
                    └────────┬────────┘
                             │
                    ┌────────▼─────────┐
                    │ Connect Wallet?  │
                    │ (Phantom/Solflare)│
                    └────┬──────┬──────┘
                         │      │
                    NO   │      │   YES
                         │      │
          ┌──────────────┘      └──────────────┐
          │                                    │
    ┌─────▼──────┐                     ┌──────▼──────┐
    │  Browse     │                     │  Wallet     │
    │  Events     │                     │  Connected  │
    │  (Read-Only)│                     │  Dashboard  │
    └─────────────┘                     └──────┬──────┘
                                               │
                        ┌──────────────────────┼──────────────────────┐
                        │                      │                      │
                   ┌────▼────┐          ┌─────▼─────┐         ┌─────▼─────┐
                   │  Event  │          │   Token   │         │   User    │
                   │ Creator │          │ Marketplace│         │  Profile  │
                   └─────────┘          └───────────┘         └───────────┘
```

---

## 🔐 Authentication Flow (Wallet Connection)

```
┌─────────────────────────────────────────────────────────────────┐
│                      WALLET CONNECTION FLOW                      │
└─────────────────────────────────────────────────────────────────┘

User Action                    Platform Response              Blockchain

    │                                  │                           │
    │  Click "Connect Wallet"          │                           │
    ├─────────────────────────────────>│                           │
    │                                  │                           │
    │                                  │  Detect installed wallets  │
    │                                  │  (Phantom, Solflare, etc) │
    │                                  │                           │
    │  <── Show wallet options ────────┤                           │
    │                                  │                           │
    │  Select Phantom                  │                           │
    ├─────────────────────────────────>│                           │
    │                                  │                           │
    │                                  │  Trigger wallet popup     │
    │  <── Wallet approval prompt ─────┤                           │
    │                                  │                           │
    │  Approve connection              │                           │
    ├─────────────────────────────────>│                           │
    │                                  │                           │
    │                                  │   Verify signature        │
    │                                  ├──────────────────────────>│
    │                                  │                           │
    │                                  │   <── Public key ─────────┤
    │                                  │                           │
    │                                  │  Create session token     │
    │                                  │  Store wallet address     │
    │                                  │  Fetch user data          │
    │                                  │                           │
    │  <── Show dashboard ─────────────┤                           │
    │      - Wallet address            │                           │
    │      - SOL balance               │                           │
    │      - Token holdings            │                           │
    │                                  │                           │

┌─────────────────────────────────────────────────────────────────┐
│                         SESSION ACTIVE                           │
│  User is now authenticated and can access all platform features  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 🎪 User Dashboard - Main Navigation

```
┌───────────────────────────────────────────────────────────────────────┐
│                     VICTORY-VERSE MAIN DASHBOARD                       │
├───────────────────────────────────────────────────────────────────────┤
│  [Logo]  Victory-Verse           [Wallet: 7x4K...mPqZ]  [1.5 SOL] [▼] │
├───────────────────────────────────────────────────────────────────────┤
│                                                                        │
│   ┌────────────┐  ┌────────────┐  ┌────────────┐  ┌────────────┐   │
│   │   BROWSE   │  │   CREATE   │  │ PARTICIPATE │  │ MARKETPLACE│   │
│   │   EVENTS   │  │   EVENT    │  │  IN EVENT  │  │            │   │
│   └─────┬──────┘  └─────┬──────┘  └─────┬──────┘  └─────┬──────┘   │
│         │               │               │               │            │
│         │               │               │               │            │
│   View all active      Create new      Register for    Buy/Sell      │
│   and upcoming        competitive      events and      event tokens  │
│   events              event with       track status                  │
│                       tokenized                                      │
│                       prizes                                         │
│                                                                        │
├───────────────────────────────────────────────────────────────────────┤
│                          QUICK STATS                                  │
│   Active Events: 47    │   Total TVL: 1,234 SOL   │  My Tokens: 8   │
└───────────────────────────────────────────────────────────────────────┘
```

---

## 🏗️ Create Event Flow (Organizer Path)

```
┌─────────────────────────────────────────────────────────────────┐
│                       EVENT CREATION FLOW                        │
└─────────────────────────────────────────────────────────────────┘

Step 1: BASIC INFORMATION
┌──────────────────────────────────┐
│ Create New Event                 │
├──────────────────────────────────┤
│ Event Name: [________________]   │
│ Category: [Gaming ▼]             │
│ Description: [_____________]     │
│            [_____________]       │
│ Start Date: [MM/DD/YYYY]         │
│ End Date: [MM/DD/YYYY]           │
│ Upload Image: [Choose File]      │
│                                  │
│        [Next: Token Setup]       │
└──────────────────────────────────┘
              ↓
              
Step 2: TOKEN ECONOMICS
┌──────────────────────────────────┐
│ Configure Token Distribution     │
├──────────────────────────────────┤
│ Total Token Supply:              │
│ [1,000,000] tokens               │
│                                  │
│ Winner Allocation: [30%]         │
│ = 300,000 tokens                 │
│                                  │
│ Event Treasury: [20%]            │
│ = 200,000 tokens                 │
│                                  │
│ Public Sale: [50%]               │
│ = 500,000 tokens                 │
│                                  │
│ Token Symbol: [____]             │
│ Initial Price: [0.01] SOL        │
│                                  │
│   [Back] [Next: Participants]    │
└──────────────────────────────────┘
              ↓
              
Step 3: PARTICIPANT MANAGEMENT
┌──────────────────────────────────┐
│ Add Participants                 │
├──────────────────────────────────┤
│ Add wallet addresses:            │
│ [__________________________]     │
│              [Add]               │
│                                  │
│ Or import CSV: [Upload File]     │
│                                  │
│ Registered Participants (5):     │
│ • 7x4K...mPqZ (Alice)           │
│ • 9bNm...kLpW (Bob)             │
│ • 2cXt...jRsM (Charlie)         │
│ • 5vWq...hTnP (Diana)           │
│ • 8pKj...fGhL (Eve)             │
│                                  │
│ Max Participants: [50]           │
│                                  │
│   [Back] [Next: Review]          │
└──────────────────────────────────┘
              ↓
              
Step 4: REVIEW & DEPLOY
┌──────────────────────────────────┐
│ Review Event Details             │
├──────────────────────────────────┤
│ Event: "Summer Gaming Showdown"  │
│ Category: Gaming                 │
│ Dates: Aug 1-15, 2025           │
│                                  │
│ Token: SUMMER (1M supply)        │
│ Winner: 300K | Treasury: 200K    │
│ Public: 500K @ 0.01 SOL          │
│                                  │
│ Participants: 5 registered       │
│                                  │
│ ⚠️ Creation Fee: 1 SOL           │
│                                  │
│ [Edit] [Create Event on Chain]   │
└──────────────────────────────────┘
              ↓
              
     BLOCKCHAIN TRANSACTION
┌──────────────────────────────────┐
│ Confirm in your wallet...        │
│                                  │
│ [Waiting for approval...]        │
│                                  │
│ ⚙️ Creating event...             │
│ ⚙️ Minting tokens...             │
│ ⚙️ Setting up vault...           │
│                                  │
└──────────────────────────────────┘
              ↓
              
        SUCCESS!
┌──────────────────────────────────┐
│ ✅ Event Created Successfully!   │
├──────────────────────────────────┤
│ Event ID: #EVT-1234              │
│ Token Mint: 5KpQ...xMnL          │
│ Vault Address: 9bTc...wRpK       │
│                                  │
│ Your event is now live!          │
│                                  │
│ [View Event] [Manage Event]      │
└──────────────────────────────────┘
```

---

## 🏆 Event Management & Winner Declaration

```
┌─────────────────────────────────────────────────────────────────┐
│                    EVENT MANAGEMENT DASHBOARD                    │
└─────────────────────────────────────────────────────────────────┘

ORGANIZER VIEW - "Summer Gaming Showdown"
┌──────────────────────────────────────────────────────────────────┐
│ [Event Image]                                                     │
│                          Summer Gaming Showdown                   │
│                          Status: ACTIVE 🟢                        │
├──────────────────────────────────────────────────────────────────┤
│                                                                   │
│  PARTICIPANTS (5)              TOKEN INFO                         │
│  ├─ Alice (7x4K...mPqZ)       Total Supply: 1M SUMMER           │
│  ├─ Bob (9bNm...kLpW)         Distributed: 0                     │
│  ├─ Charlie (2cXt...jRsM)     Available: 1M                      │
│  ├─ Diana (5vWq...hTnP)       Price: 0.01 SOL                    │
│  └─ Eve (8pKj...fGhL)                                            │
│                                                                   │
│  [Add Participant] [Remove]    ANALYTICS                         │
│                                Page Views: 1,247                  │
│  WINNER SELECTION             Interested Users: 89               │
│  Select Winner: [Choose ▼]    Social Shares: 34                  │
│                                                                   │
│          [Declare Winner]                                         │
│                                                                   │
└──────────────────────────────────────────────────────────────────┘

              ↓ (Organizer selects Alice and clicks Declare Winner)

┌──────────────────────────────────────────────────────────────────┐
│ Confirm Winner Declaration                                        │
├──────────────────────────────────────────────────────────────────┤
│ You are about to declare Alice as the winner                     │
│                                                                   │
│ This will trigger:                                                │
│ ✓ Transfer 300,000 SUMMER tokens to Alice                       │
│ ✓ Transfer 200,000 SUMMER to event treasury                     │
│ ✓ Mint NFT trophy to Alice's wallet                             │
│ ✓ Make 500,000 SUMMER available for public purchase             │
│ ✓ Change event status to COMPLETED                              │
│                                                                   │
│ ⚠️ This action cannot be undone!                                 │
│                                                                   │
│            [Cancel] [Confirm & Execute]                          │
└──────────────────────────────────────────────────────────────────┘

              ↓ (Smart contract execution)

SMART CONTRACT EXECUTION
┌──────────────────────────────────────────────────────────────────┐
│ Processing...                                                     │
│                                                                   │
│ ✅ Token distribution to winner (300K)                           │
│ ✅ Token distribution to treasury (200K)                         │
│ ✅ NFT trophy minted (Token ID: #TROPHY-1234)                    │
│ ✅ Public sale activated (500K available)                        │
│ ✅ Event status updated to COMPLETED                             │
│                                                                   │
│ Transaction: https://solscan.io/tx/5xKm...pQwR                   │
│                                                                   │
│                    [View Event Page]                             │
└──────────────────────────────────────────────────────────────────┘

COMPLETED EVENT VIEW
┌──────────────────────────────────────────────────────────────────┐
│ Summer Gaming Showdown - COMPLETED 🏆                            │
├──────────────────────────────────────────────────────────────────┤
│ Winner: Alice (7x4K...mPqZ)                                      │
│ Prize: 300,000 SUMMER tokens                                     │
│                                                                   │
│ [Trophy NFT Preview]          TOKEN NOW LIVE                     │
│                               Current Price: 0.012 SOL (+20%)    │
│                               24h Volume: 45 SOL                 │
│                               Holders: 23                        │
│                                                                   │
│                               [Buy SUMMER Tokens]                │
└──────────────────────────────────────────────────────────────────┘
```

---

## 🛒 Marketplace Flow (Fan/Token Holder Path)

```
┌─────────────────────────────────────────────────────────────────┐
│                     TOKEN MARKETPLACE HOME                       │
└─────────────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────────────┐
│ Marketplace                                    [Search: _______] │
├──────────────────────────────────────────────────────────────────┤
│                                                                   │
│ Filters: [All Events ▼] [All Categories