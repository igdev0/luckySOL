import {PublicKey} from '@solana/web3.js';
import {vec, field, variant, fixedArray, serialize, serializer, BinaryWriter,} from '@dao-xyz/borsh';

export class PoolStorageData {
  @field({type: "u64"})
  ticket_price: BigInt;
  @field({type: "u64"})
  draft_count: BigInt;
  @field({type: "u64"})
  initial_amount: BigInt;

  constructor(data: PoolStorageData) {
    this.ticket_price = data.ticket_price
    this.draft_count = data.draft_count
    this.initial_amount = data.initial_amount
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
  proof: number[];
  @field({type: vec("u64")})
  ticket_indices: bigint[];
  @field({type: vec(fixedArray("u8", 32))})
  tickets: Uint8Array[];
  @field({type: fixedArray('u8', 32)})
  address: Uint8Array;
  @field({type: fixedArray('u8', 32)})
  token_account: Uint8Array;
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
  @field({
    type: fixedArray('u8', 32)
  })
  merkle_root: Uint8Array
  @field({type: "u64"})
  total_tickets: string

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
export class PlayerWithdraw {
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
  accountData: Array<DraftWinner>
  constructor(data: DraftWinner[]) {
    this.accountData = data;
  }
}

@variant(5)
export class ClosePlayerAccount {}

export type Instruction = InitializePool | Deposit | PlayerWithdraw | PurchaseTicket | SelectWinnersAndAirdrop | ClosePlayerAccount;