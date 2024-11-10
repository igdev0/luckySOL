import {PublicKey} from '@solana/web3.js';

export const PoolStorageSeed = {
  StakePool: "StakePool",
  ReceiptMint: "ReceiptMint",
  StakeHouse: "StakeHouse",
  PlayerAccount: "PlayerAccount",
  PlayerTokenAccount: "PlayerTokenAccount",
} as const

export const PROGRAM_ID = new PublicKey("1cky9mEdiuQ8wNCcw1Z7pXuxF9bsdxej95Gf69XydoA");

