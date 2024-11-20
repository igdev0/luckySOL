import { Test, TestingModule } from '@nestjs/testing';
import { LuckyDraftController } from './lucky-draft.controller';

describe('LuckyDraftController', () => {
  let controller: LuckyDraftController;

  beforeEach(async () => {
    const module: TestingModule = await Test.createTestingModule({
      controllers: [LuckyDraftController],
    }).compile();

    controller = module.get<LuckyDraftController>(LuckyDraftController);
  });

  it('should be defined', () => {
    expect(controller).toBeDefined();
  });
});
