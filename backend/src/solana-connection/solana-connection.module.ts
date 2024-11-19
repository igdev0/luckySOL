import { DynamicModule, Global, Module, ModuleMetadata } from '@nestjs/common';
import {
  SOLANA_CONNECTION_OPTIONS,
  SolanaConnectionOptions,
  SolanaConnectionService,
} from './solana-connection.service';

export interface SolanaConnectionModuleAsyncOptions
  extends Pick<ModuleMetadata, 'imports'> {
  useFactory: (...args: any[]) => SolanaConnectionOptions;
  inject: any[];
}

@Global()
@Module({
  providers: [SolanaConnectionService],
  exports: [SolanaConnectionService],
})
export class SolanaConnectionModule {
  static forRoot(options: SolanaConnectionOptions): DynamicModule {
    return {
      module: SolanaConnectionModule,
      providers: [
        {
          provide: SOLANA_CONNECTION_OPTIONS,
          useValue: options,
        },
      ],
      exports: [SOLANA_CONNECTION_OPTIONS],
    };
  }

  static forRootAsync(
    options: SolanaConnectionModuleAsyncOptions,
  ): DynamicModule {
    return {
      module: SolanaConnectionModule,
      providers: [
        {
          provide: SOLANA_CONNECTION_OPTIONS,
          useFactory: options.useFactory,
          inject: options.inject ?? [],
        },
      ],
      exports: [SOLANA_CONNECTION_OPTIONS],
    };
  }
}
