import * as borsh from 'borsh';
import { PublicKey } from '@solana/web3.js';

export class DraftWinner {
  amount: bigint;
  proof: Uint8Array;
  ticketIndices: number[];
  tickets: Uint8Array[];
  address: PublicKey;
  tokenAccount: PublicKey;

  constructor(fields: { amount: bigint; proof: Uint8Array; ticketIndices: number[]; tickets: Uint8Array[]; address: PublicKey; tokenAccount: PublicKey }) {
    this.amount = fields.amount;
    this.proof = fields.proof;
    this.ticketIndices = fields.ticketIndices;
    this.tickets = fields.tickets;
    this.address = fields.address;
    this.tokenAccount = fields.tokenAccount;
  }
  // Define the schema with exact field structure required by borsh
  static schema: borsh.Schema = {
    struct: {
      "amount": "u64",
      "proof": {
        array: {
          type: "u8"
        }
      },
      ticketIndices: {
        array: {
          type: "u32"
        },
      },
      tickets: {
        array: {
          type: "u8"
        }
      },
      address: "u64",
      tokenAccount: "u64"
    }
  };
}