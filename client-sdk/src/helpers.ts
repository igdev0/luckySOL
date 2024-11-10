import {PublicKey} from '@solana/web3.js';
import {PoolStorageSeed, PROGRAM_ID} from './constants.js';

export function findPoolStorageSeed(address: PublicKey) {
  const seeds = [
    Uint8Array.from(Buffer.from(PoolStorageSeed.StakePool)),
    Uint8Array.from(address.toBuffer()),
  ];
    return PublicKey.findProgramAddressSync(seeds, PROGRAM_ID);
}