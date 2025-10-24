# Victory-Verse User Stories

## Overview
User stories for Victory-Verse, organized by user type .

---

## User Types

1. **Event Organizer** - Creates and manages  events
2. **Participant/Winner** - Competes in events and receives rewards
3. **Fan/Token Holder** - Supports winners by purchasing and trading tokens
4. **Visitor** - Exploring the platform without wallet connection

---

## Core User Stories

### 1. Wallet Connection & Authentication

#### Story A: Connect Wallet
**As a** visitor  
**When I** click the "Connect Wallet" button and approve the connection in my Solana wallet  
**Then** I am logged into the platform and can see my wallet address displayed

**Acceptance Criteria:**
- Wallet connection supports Phantom, Solflare, and Backpack wallets
- User sees a clear success message after connection
- User's SOL balance is displayed
- Navigation menu updates to show logged-in options
- Session persists until user manually disconnects

#### Story B: Disconnect Wallet
**As a** logged-in user  
**When I** click "Disconnect Wallet" in the menu  
**Then** I am logged out and returned to the visitor view

**Acceptance Criteria:**
- All personal data is cleared from the interface
- User is redirected to the home page
- A confirmation message appears before disconnecting
- User must reconnect wallet to access protected features

---

### 2. Event Organizer Stories

#### Story A: Create New Event
**As an** event organizer  
**When I** navigate to "Create Event" and fill in the event details (name, description, dates, prize structure)  
**Then** a new event is created on the blockchain with a unique token

**Acceptance Criteria:**
- Form includes: event name, description, start/end dates, image upload
- Token economics configuration: total supply, winner %, treasury %, public sale %
- Event creation fee (1 SOL) is clearly displayed and deducted
- Confirmation screen shows all details before submitting to blockchain
- Event appears in "My Events" dashboard immediately after creation
- Unique event ID and token mint address are generated
- Event status is set to "Registration"

#### Story B: Register Participants
**As an** event organizer  
**When I** add participant wallet addresses to my event  
**Then** those participants are registered and can compete

**Acceptance Criteria:**
- Can add participants individually or bulk import via CSV
- Each participant wallet is validated before adding
- Maximum participant limit is enforced (if set)
- Participants receive notification of registration
- Can remove participants before event starts
- Cannot modify participant list after event status changes to "Active"

#### Story C: Declare Winner
**As an** event organizer  
**When I** select a winner from the registered participants and click "Declare Winner"  
**Then** tokens are automatically distributed and an NFT trophy is minted

**Acceptance Criteria:**
- Can only declare winner from registered participants
- Confirmation dialog shows token distribution breakdown
- Winner receives their token allocation (e.g., 30% of total supply)
- Winner automatically receives NFT trophy in their wallet
- Event treasury receives allocated tokens (e.g., 20%)
- Remaining tokens move to "Available for Public Sale"
- Event status changes to "Completed"
- Winner announcement is visible on event page
- Transaction hash is displayed for verification


---

### 3. Participant/Winner Stories

#### Story A: View Available Events
**As a** participant  
**When I** navigate to the "Events" page  
**Then** I see all active and upcoming events I can participate in

**Acceptance Criteria:**
- Each event shows: name, image, dates, prize structure, participant count
- Can filter by: status (registration/active/completed), category, date
- Can search by event name or organizer
- Past events show winner information

#### Story B: Participate in Event
**As a** registered participant  
**When I** compete in an event and am declared the winner  
**Then** I receive my token allocation and NFT trophy in my wallet

**Acceptance Criteria:**
- Tokens appear in wallet within 30 seconds of winner declaration
- NFT trophy appears in wallet's NFT collection
- Winner badge/indicator appears on user profile
- Notification confirms successful token transfer
- Token amount matches the promised allocation percentage
- Tokens have initial lock period clearly displayed (e.g., "30 days until tradeable")

#### Story C: View My Achievements
**As a** winner  
**When I** navigate to "My Achievements"  
**Then** I see all events I've won and my trophy NFTs

**Acceptance Criteria:**
- All won events are listed with dates and prize amounts
- Trophy NFTs are displayed with artwork and metadata
- Shows current value of tokens received
- Shows trading volume of event tokens
- Can share achievements on social media
- Trophy gallery is publicly viewable via profile link

---

### 4. Fan/Token Holder Stories

#### Story A: Browse Event Tokens
**As a** fan  
**When I** visit the "Marketplace" page  
**Then** I see all available event tokens I can purchase

**Acceptance Criteria:**
- Tokens are displayed with: event name, winner name, current price, 24h change
- Can sort by: newest, price, volume, market cap
- Can filter by: event category, date range
- Each token shows available supply for purchase
- Clicking a token shows detailed page with price chart and event history

#### Story B: Buy Event Tokens
**As a** fan  
**When I** select an event token and specify the amount to buy  
**Then** tokens are transferred to my wallet after payment

**Acceptance Criteria:**
- Clear display of: token price, amount to buy, total cost in SOL
- Transaction fee estimate is shown before confirmation
- Must confirm transaction in wallet
- Tokens appear in wallet within 30 seconds
- Purchase is recorded in transaction history
- Remaining available supply updates immediately
- Cannot buy more than available supply

#### Story C: Sell Event Tokens
**As a** token holder  
**When I** list my tokens for sale in the marketplace  
**Then** other users can purchase them at my set price

**Acceptance Criteria:**
- Can set custom price per token
- Can choose to sell partial or full holdings
- Listing appears immediately in marketplace
- Can cancel listing at any time before sale
- Tokens are locked while listed (cannot be transferred elsewhere)
- Receive SOL payment automatically when tokens are purchased
- 2.5% trading fee is deducted and clearly displayed

#### Story D: Stake Tokens
**As a** token holder  
**When I** stake my event tokens with a lock period  
**Then** I earn staking rewards over time

**Acceptance Criteria:**
- Can choose lock period: 30 days, 90 days, 180 days, 365 days
- APY (annual percentage yield) is clearly displayed for each option
- Shows estimated rewards at end of lock period
- Confirmation dialog explains tokens will be locked and untradeable
- Staked amount appears in "My Staking" dashboard
- Shows countdown timer until unlock date
- Rewards accumulate and are claimable daily
- Can unstake after lock period ends
- Early unstaking option available with penalty clearly shown

