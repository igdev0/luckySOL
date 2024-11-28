import { Inject, Injectable, OnModuleInit } from '@nestjs/common';
import { Commitment, Connection, ConnectionConfig } from '@solana/web3.js';

export interface SolanaConnectionOptions {
  endpoint: string;
  commitmentOrConfig: Commitment | ConnectionConfig;
}

export const SOLANA_CONNECTION_OPTIONS = 'SOLANA_CONNECTION_OPTIONS';

@Injectable()
export class SolanaConnectionService implements OnModuleInit {
  private connection: Connection;

  constructor(
    @Inject(SOLANA_CONNECTION_OPTIONS)
    private readonly options: SolanaConnectionOptions,
  ) {}

  onModuleInit() {
    const endpoint = this.options.endpoint;
    const commitmentOrConfig = this.options.commitmentOrConfig;

    this.connection = new Connection(endpoint, commitmentOrConfig);

    console.log('Solana Connection initialized');
  }

  public getConnection(): Connection {
    return this.connection;
  }
}
