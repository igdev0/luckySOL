import { Test, TestingModule } from '@nestjs/testing';
import { SolanaConnectionService } from './solana-connection.service';

describe('SolanaConnectionService', () => {
  let service: SolanaConnectionService;

  beforeEach(async () => {
    const module: TestingModule = await Test.createTestingModule({
      providers: [SolanaConnectionService],
    }).compile();

    service = module.get<SolanaConnectionService>(SolanaConnectionService);
  });

  it('should be defined', () => {
    expect(service).toBeDefined();
  });
});
