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

