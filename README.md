# Ezstream 🚀

<p align="center">
<img src="https://github.com/Stream-Tips/Ezstream_SC/blob/main/logo192.png?raw=true" alt="Ezstream Logo" width="200"/>

![Solana](https://img.shields.io/badge/Blockchain-Solana-green.svg)

Ezstream is a cutting-edge decentralized application (dApp) built on the [Solana blockchain](https://solana.com/). It empowers users to seamlessly tip live stream creators with SOLANA, fostering a direct and transparent connection between viewers and their favorite content creators.

## Table of Contents

- [✨ Features](#-features)
- [🎯 Motivation](#-motivation)
- [🚀 Getting Started](#-getting-started)
  - [🔧 Prerequisites](#-prerequisites)
- [🛠️ Usage](#️-usage)
- [📂 Components](#-components)
- [📜 Smart Contract](#-smart-contract)
- [🔗 API Endpoints](#-api-endpoints)
- [🔄 WebSocket Events](#-websocket-events)
- [📚 References](#-references)

## ✨ Features

- **💸 Live Stream Tipping**: Empower viewers to tip live stream creators directly with SOL tokens.
- **📊 Creator Dashboard**: Enable creators to monitor their account balance and effortlessly withdraw funds.
- **📚 Live Stream Directory**: Browse a comprehensive list of active live streams on the platform.
- **💬 Chat Panel**: Engage with real-time chat functionality during live streams.
- **🔗 Wallet Integration**: Seamlessly connect with Solana wallets using [`@solana/wallet-adapter-react`](https://github.com/solana-labs/wallet-adapter).
- **🔐 Secure Transactions**: Ensure safe and transparent transactions on the Solana blockchain.

## 🎯 Motivation

In the evolving landscape of live streaming, creators seek effective and direct ways to monetize their content. Ezstream bridges the gap between viewers and creators by leveraging the power of blockchain technology. This dApp not only facilitates instant and secure tipping but also fosters a more engaged and supportive community.

## 🚀 Getting Started

### 🔧 Prerequisites

Before you begin, ensure you have met the following requirements:

- **Node.js** (v14 or later)
- **npm** or **yarn**
- **Solana CLI**: [Installation Guide](https://docs.solana.com/cli/install-solana-cli-tools)
- **Rust**: [Installation Guide](https://www.rust-lang.org/tools/install), required for building the smart contract.

## 🛠️ Usage

1. **Connect your Solana wallet**: Click on the "Connect Wallet" button to link your Solana wallet.

2. **Create a Creator Account**: If you are a content creator, set up your creator profile to start receiving tips.

3. **Start a Live Stream**: Initiate a live stream and share the link with your audience.

4. **Viewers can tip**: Audiences can tip creators using the "Tip Creator" feature during the live stream.

5. **Creators can withdraw**: Access the "Creator Dashboard" to view your balance and withdraw your funds securely.

## 📂 Components

### `CreatorSection.tsx`

Handles the creator's dashboard, allowing them to view their balance and withdraw funds.

### `LiveStreamPage.tsx`

Dedicated page for viewing a live stream, featuring the video player and chat panel.

### `TipPanel.tsx`

Facilitates tipping by allowing viewers to send SOL tokens to creators during a live stream.

### `ChatPanel.tsx`

Offers real-time chat functionality, enhancing viewer interaction during streams.

### `ConnectWalletButton.tsx`

Manages wallet connections using `@solana/wallet-adapter-react` for seamless integration.

## 📜 Smart Contract

Our smart contract, written in Rust, resides on the Solana blockchain. It orchestrates key functionalities such as account creation, tipping, and withdrawals.

### Key Functions

- **initialize_creator_account**: Initializes a new creator account on the blockchain.
- **process_tip**: Facilitates tipping from a viewer to a creator securely.
- **process_withdraw**: Enables creators to withdraw their accumulated tips.

## 🔗 API Endpoints

Base URL: `http://localhost:3000/api`

### Authentication

All authenticated endpoints require a JWT token in the Authorization header:
`Authorization: Bearer <token>`

### 1. User Authentication

#### POST /auth/connect

Authenticate a user with their wallet address.

json{ "walletAddress": "string"}**Response:**json
{
"token": "string",
"creatorId": "string"
}


## 🔄 WebSocket Events

WebSocket URL: `ws://localhost:3000/ws?token=<jwt_token>`

### Client to Server Events

1. **Join Stream**

   ```json
   {
     "type": "join",
     "streamId": "string"
   }
   ```

2. **Leave Stream**

   ```json
   {
     "type": "leave",
     "streamId": "string"
   }
   ```

3. **Send Chat Message**

   ```json
   {
     "type": "chat",
     "streamId": "string",
     "message": "string"
   }
   ```

4. **Start Stream (Creator only)**

   ```json
   {
     "type": "startStream",
     "streamId": "string"
   }
   ```

5. **End Stream (Creator only)**

   ```json
   {
     "type": "endStream",
     "streamId": "string"
   }
   ```

### Server to Client Events

1. **Chat Message**

   ```json
   {
     "type": "chat",
     "streamId": "string",
     "senderId": "string",
     "message": "string",
     "timestamp": "string (ISO date)"
   }
   ```

2. **User Joined**

   ```json
   {
     "type": "userJoined",
     "streamId": "string",
     "userId": "string"
   }
   ```

3. **User Left**

   ```json
   {
     "type": "userLeft",
     "streamId": "string",
     "userId": "string"
   }
   ```

4. **Stream Started**

   ```json
   {
     "type": "streamStarted",
     "streamId": "string"
   }
   ```

5. **Stream Ended**

   ```json
   {
     "type": "streamEnded",
     "streamId": "string"
   }
   ```

6. **Tip Received**

   ```json
   {
     "type": "tipReceived",
     "streamId": "string",
     "amount": "number",
     "fromUserId": "string"
   }
   ```

## 📚 References

- [Current & Future State Ezstream,](https://ezstream.gitbook.io/ezstream) - Ezstream