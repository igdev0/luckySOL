import {PublicKey} from '@solana/web3.js';
import {vec, VecKind, field, variant,} from '@dao-xyz/borsh';

export class PoolStorageData {
  @field({type: "u64"})
  initial_amount: bigint;
  @field({type: "u64"})
  ticket_price: bigint;
  @field({type: "u64"})
  draft_count: bigint;

  constructor(data: PoolStorageData) {
    this.draft_count = data.draft_count
    this.initial_amount = data.initial_amount
    this.ticket_price = data.ticket_price
  }
}

@variant(0)
export class InitializePool {
  @field({type: PoolStorageData})
  poolData: PoolStorageData;
  constructor(poolData: PoolStorageData) {
    this.poolData = poolData
  }
}

export class DraftWinner {
  @field({type: "u64"})
  amount: bigint;
  @field({type: vec("u8")})
  proof: Uint8Array;
  @field({type: vec("u32")})
  ticket_indices: number[];
  @field({type: vec(vec("u8"))})
  tickets: Uint8Array[];
  @field({type: "u64"})
  address: PublicKey;
  @field({type: "u64"})
  token_account: PublicKey;
  constructor(fields: DraftWinner) {
    this.amount = fields.amount;
    this.proof = fields.proof;
    this.ticket_indices = fields.ticket_indices;
    this.tickets = fields.tickets;
    this.address = fields.address;
    this.token_account = fields.token_account;
  }
}

export type Instruction = InitializePool