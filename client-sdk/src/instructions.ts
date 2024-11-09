import * as borsh from 'borsh';
import {PublicKey} from '@solana/web3.js';

export interface PoolStorageDataType {
  initialAmount: bigint,
  ticketPrice: bigint,
  draftCount: bigint
}

export class PoolStorageData {
  initialAmount: bigint;
  ticketPrice: bigint;
  draftCount: bigint;

  constructor(fields: PoolStorageDataType) {
    this.draftCount = fields.draftCount;
    this.ticketPrice = fields.ticketPrice;
    this.initialAmount = fields.initialAmount;
  }

  static schema: borsh.Schema = {
    struct: {
      initial_amount: "u64",
      ticket_price: "u64",
      draft_count: "u64",
    }
  };
}

export class DraftWinner {
  amount: bigint;
  proof: Uint8Array;
  ticketIndices: number[];
  tickets: Uint8Array[];
  address: PublicKey;
  tokenAccount: PublicKey;

  constructor(fields: {
    amount: bigint;
    proof: Uint8Array;
    ticketIndices: number[];
    tickets: Uint8Array[];
    address: PublicKey;
    tokenAccount: PublicKey
  }) {
    this.amount = fields.amount;
    this.proof = fields.proof;
    this.ticketIndices = fields.ticketIndices;
    this.tickets = fields.tickets;
    this.address = fields.address;
    this.tokenAccount = fields.tokenAccount;
  }

  static schema: borsh.Schema = {
    struct: {
      "amount": "u64",
      "proof": {
        array: {
          type: "u8"
        }
      },
      ticket_indices: {
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
      token_account: "u64"
    }
  };
}