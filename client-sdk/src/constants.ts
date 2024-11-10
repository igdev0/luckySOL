import {PublicKey, SYSVAR_RENT_PUBKEY} from '@solana/web3.js';

export const PoolStorageSeed = {
  StakePool: "StakePool",
  ReceiptMint: "ReceiptMint",
  StakeHouse: "StakeHouse",
  PlayerAccount: "PlayerAccount",
  PlayerTokenAccount: "PlayerTokenAccount",
} as const

export const PROGRAM_ID = new PublicKey("1cky9mEdiuQ8wNCcw1Z7pXuxF9bsdxej95Gf69XydoA");
export const TOKEN_PROGRAM_ID = new PublicKey("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");
export const SYSTEM_PROGRAM_ID = new PublicKey("11111111111111111111111111111111");