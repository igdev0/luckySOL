import {PublicKey} from '@solana/web3.js';
import {PoolStorageSeed, PROGRAM_ID} from './constants.js';

export function findPoolStoragePDA(address: PublicKey) {
  const seeds = [
    Uint8Array.from(Buffer.from(PoolStorageSeed.StakePool)),
    Uint8Array.from(address.toBuffer()),
  ];
    return PublicKey.findProgramAddressSync(seeds, PROGRAM_ID);
}

export function findReceiptPoolMintPDA(address: PublicKey) {
  const seeds = [
    Uint8Array.from(Buffer.from(PoolStorageSeed.ReceiptMint)),
    Uint8Array.from(address.toBuffer()),
  ];
  return PublicKey.findProgramAddressSync(seeds, PROGRAM_ID);
}

export function findPlayerAccountPDA(address: PublicKey) {
  const seeds = [
      Uint8Array.from(Buffer.from(PoolStorageSeed.PlayerAccount)),
      Uint8Array.from(address.toBuffer())
  ];
  return PublicKey.findProgramAddressSync(seeds, PROGRAM_ID);
}

export function findPlayerTokenAccountPDA(address: PublicKey) {
  const seeds = [
      Uint8Array.from(Buffer.from(PoolStorageSeed.PlayerTokenAccount)),
      Uint8Array.from(address.toBuffer())
  ];
  return PublicKey.findProgramAddressSync(seeds, PROGRAM_ID);
}