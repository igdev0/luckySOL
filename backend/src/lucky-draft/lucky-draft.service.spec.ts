import { Test, TestingModule } from '@nestjs/testing';
import { LuckyDraftService } from './lucky-draft.service';

describe('LuckyDraftService', () => {
  let service: LuckyDraftService;

  beforeEach(async () => {
    const module: TestingModule = await Test.createTestingModule({
      providers: [LuckyDraftService],
    }).compile();

    service = module.get<LuckyDraftService>(LuckyDraftService);
  });

  it('should be defined', () => {
    expect(service).toBeDefined();
  });
});
