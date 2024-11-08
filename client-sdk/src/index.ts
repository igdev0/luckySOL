import {DraftWinner} from './instructions.js';
import {AccountMeta, PublicKey} from '@solana/web3.js';
import * as borsh from "borsh"
export function processDraftWinner(amount: bigint, address: PublicKey, proof: Uint8Array, tickets: Uint8Array[], ticketIndices: number[], tokenAccount: PublicKey, ) {
  const draftWinner = new DraftWinner({
    amount,
    address,
    proof,
    tickets,
    ticketIndices,
    tokenAccount
  })
  const serialised = borsh.serialize(DraftWinner.schema, draftWinner);

  const accounts:AccountMeta[] = [];
}