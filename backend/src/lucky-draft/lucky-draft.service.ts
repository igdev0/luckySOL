import { Inject, Injectable, OnApplicationBootstrap } from '@nestjs/common';
import { Cron, CronExpression } from '@nestjs/schedule';
import { Repository } from 'typeorm';
import { Ticket } from '../ticket/entities/ticket.entity';
import { InjectRepository } from '@nestjs/typeorm';
import { SolanaConnectionService } from '../solana-connection/solana-connection.service';
import { LuckyDraftEntity } from './entities/lucky-draft.entity';
import { ConfigService } from '@nestjs/config';
import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  TransactionMessage,
  VersionedTransaction,
} from '@solana/web3.js';
import {
  findPoolStoragePDA,
  InitializePool,
  POOL_STORAGE_DATA_LENGTH,
  PoolStorageData,
  processPoolInitializationInstruction,
} from 'lucky-sol-sdk';

@Injectable()
export class LuckyDraftService implements OnApplicationBootstrap {
  private connection: Connection;
  private poolStoragePDA: PublicKey;
  private poolAuthorityKeypair: Keypair;

  constructor(
    @InjectRepository(Ticket)
    private readonly ticketRepository: Repository<Ticket>,
    @Inject(SolanaConnectionService)
    private readonly solanaConnection: SolanaConnectionService,
    @InjectRepository(LuckyDraftEntity)
    private readonly luckyDraftEntityRepository: Repository<LuckyDraftEntity>,
    @Inject(ConfigService) private readonly config: ConfigService,
  ) {}

  async onApplicationBootstrap(): Promise<any> {
    const connection = this.solanaConnection.getConnection();
    const recentHash = await connection.getLatestBlockhash();
    const seed = Uint8Array.from(
      JSON.parse(this.config.get('POOL_AUTHORITY_KEYPAIR')),
    );
    const authority = Keypair.fromSecretKey(seed);
    const [poolStoragePDA] = findPoolStoragePDA(authority.publicKey);

    // Store those privately for convenience.
    this.poolStoragePDA = poolStoragePDA;
    this.connection = connection;
    this.poolAuthorityKeypair = authority;

    const poolStorageAccount = await connection.getAccountInfo(poolStoragePDA);
    if (!!poolStorageAccount) {
      return null;
    }

    const poolData = new PoolStorageData({
      ticket_price: BigInt(0.5 * LAMPORTS_PER_SOL),
      draft_count: BigInt(0),
      initial_amount: BigInt(10 * LAMPORTS_PER_SOL),
    });
    const data = new InitializePool(poolData);
    const initialize = processPoolInitializationInstruction(
      data,
      authority.publicKey,
    );
    const vmsg = new TransactionMessage({
      payerKey: authority.publicKey,
      recentBlockhash: recentHash.blockhash,
      instructions: [initialize],
    }).compileToV0Message();

    const tx = new VersionedTransaction(vmsg);
    tx.sign([authority]);
    await connection.sendTransaction(tx);
  }

  splitLotteryPrize(totalPrize, weights = []) {
    if (totalPrize <= 0) {
      throw new Error('Total prize must be greater than 0.');
    }

    if (!Array.isArray(weights) || weights.length === 0) {
      throw new Error('Weights must be an array with at least one value.');
    }

    const totalWeight = weights.reduce((acc, weight) => {
      if (weight <= 0) throw new Error('Weights must be positive numbers.');
      return acc + weight;
    }, 0);

    return weights.map((weight) => (totalPrize * weight) / totalWeight);
  }

  @Cron(CronExpression.EVERY_SECOND)
  async handleDraft() {
    const luckyNumbers = this.generateLotteryNumbers();
    const totalPrize = await this.getTotalPoolPrize();

    const result = this.splitLotteryPrize(totalPrize, [1, 1, 1, 1]);
    console.log({ result, totalPrize });
    const luckyNumbersDraft = this.luckyDraftEntityRepository.create({
      total_prizes_won: BigInt(0), // change this to the real prize won
      lucky_draft: JSON.parse(JSON.stringify(luckyNumbers)),
    });

    // Save the drafted lucky numbers
    await this.luckyDraftEntityRepository.save(luckyNumbersDraft);

    const entities = await this.ticketRepository.find({
      where: { status: 'PendingCreation' },
    });

    const entitiesMatching = entities.map<
      Ticket & { matches: (number | null)[][] }
    >((entity) => {
      const matches = (
        JSON.parse(JSON.stringify(entity.lucky_draft)) as [number[]]
      ).map((luckyTicket) => {
        return luckyTicket.map<number | null>((luckyNumber) =>
          luckyNumbers.has(luckyNumber) ? luckyNumber : null,
        );
      });

      return {
        ...entity,
        matches,
        matches_count: matches.flatMap(
          (item) => item.filter((v) => v !== null).length,
        ),
      };
    });

    // @todo:
    // - Verify the numbers and check their winning amount.🕣
    // - Interact with the luckySOL contract to airdrop the rewards.
  }

  async findAll(page, limit) {
    const [data, total] = await this.luckyDraftEntityRepository.findAndCount({
      skip: (page - 1) * limit,
      take: limit,
      order: { created_at: 'ASC' },
    });

    return {
      data,
      total,
      page,
      limit,
    };
  }

  private async getTotalPoolPrize() {
    const totalAmount = await this.connection.getBalance(this.poolStoragePDA);
    const minimumBalance =
      await this.connection.getMinimumBalanceForRentExemption(
        POOL_STORAGE_DATA_LENGTH,
      );
    return totalAmount - minimumBalance;
  }

  private generateRandomNumber(x = 0, y = 49) {
    if (x > y) {
      throw new Error(
        'The first parameter (x) must be less than or equal to the second parameter (y).',
      );
    }
    return Math.floor(Math.random() * (y - x + 1)) + x;
  }

  private generateLotteryNumbers() {
    const x = 0;
    const y = 49;
    const luckyNumbers = new Set<number>();
    while (luckyNumbers.size !== 7) {
      luckyNumbers.add(this.generateRandomNumber(x, y));
    }
    return luckyNumbers;
  }
}
