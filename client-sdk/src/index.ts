import {Connection, Version} from '@solana/web3.js';

export class SolanaSDK {
  private connection: Connection;

  constructor(rpcUrl: string) {
    this.connection = new Connection(rpcUrl);
  }

  public async getVersion(): Promise<Version> {
    return this.connection.getVersion();
  }
}