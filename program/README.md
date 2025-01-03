# LuckySOL

A decentralized lottery system built on the Solana blockchain, offering a trustless, efficient, and transparent gaming
experience. This program employs a hybrid on-chain/off-chain architecture to optimize performance and scalability.

## Key Features

### For Users:

- **Purchase Tickets:**  
  Users can buy tickets to enter the lottery using SOL. Each ticket is represented as part of a
  Merkle tree, with the Merkle root saved on-chain for efficient verification. Upon purchasing a ticket, users receive a
  **pool token receipt**, which serves as proof of participation in the lottery.

- **Withdraw Rewards:**  
  Winning users can claim their rewards directly from the program using cryptographic proofs derived from the Merkle
  tree.

### For Pool owners:

- **Initialize Pool:**  
  Admins can set up a lottery pool, defining parameters such as ticket price, reward distribution, and lottery duration.
  During this process, a **pool token receipt** is also initialized, enabling user participation and reward tracking.

- **Select Winners:**  
  Admins utilize a fair and verifiable mechanism to select winners.

- **Airdrop Rewards:**  
  Rewards are distributed seamlessly to the winners.

## Hybrid Ticket System

- **Off-Chain Ticket Storage:**  
  Tickets are stored off-chain, significantly reducing on-chain storage costs.

- **Merkle Root Verification:**  
  A Merkle root is stored on-chain to ensure all tickets are verifiable and immutable.

- **Cryptographic Proofs:**  
  Users provide Merkle proofs when interacting with the program (e.g., claiming rewards), ensuring security and
  trustworthiness.

## Pool Token Receipt

- **What It Is:**  
  A unique token issued to users upon ticket purchase, serving as proof of participation in the lottery.

- **Functionality:**
    - Tracks the number of tickets purchased by a user.
    - Can be used in future reward distributions or governance mechanisms.

- **Initialization:**  
  The pool token receipt is created and initialized during the pool setup, ensuring that all participants receive their
  receipt seamlessly when entering the lottery.

## Benefits

By leveraging Merkle tree-based verification and Solana's high-speed, low-cost infrastructure, this program offers:

- **Scalability:** Off-chain ticket storage reduces on-chain data requirements.
- **Security:** Immutable and verifiable tickets through Merkle tree mechanisms.
- **Transparency:** On-chain processes ensure fairness and trustlessness.
- **Enhanced User Experience:** Pool token receipts provide a seamless mechanism for tracking participation and rewards.

---

This program is a robust solution for decentralized lotteries, providing a seamless and efficient user experience while
maintaining the integrity and transparency expected in blockchain applications.