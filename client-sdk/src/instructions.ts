import {PublicKey} from '@solana/web3.js';
import {vec, VecKind, field, variant, fixedArray,} from '@dao-xyz/borsh';

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

export class TicketAccountData {
  @field({type: fixedArray("u8", 32)})
  merkle_root: number[]
  @field({type: "u64"})
  total_tickets: bigint

  constructor(fields: TicketAccountData) {
    this.merkle_root = fields.merkle_root;
    this.total_tickets = fields.total_tickets;
  }
}

@variant(1)
export class Deposit {
  @field({type: "u64"})
  amount: bigint
  constructor(amount: bigint) {
    this.amount = amount;
  }
}

@variant(2)
export class Withdraw {
  @field({type: "u64"})
  amount: bigint
  constructor(amount: bigint) {
    this.amount = amount;
  }
}

@variant(3)
export class PurchaseTicket {
  @field({type: TicketAccountData})
  accountData: TicketAccountData;

  constructor(data: TicketAccountData) {
    this.accountData = data;
  }
}

@variant(4)
export class SelectWinnersAndAirdrop {
  @field({type: vec(DraftWinner)})
  data: DraftWinner[]
  constructor(data: DraftWinner[]) {
    this.data = data;
  }
}

export type Instruction = InitializePool | Deposit | Withdraw | PurchaseTicket | SelectWinnersAndAirdrop