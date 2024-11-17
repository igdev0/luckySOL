import { Module } from '@nestjs/common';
import { LuckyDraftService } from './lucky-draft.service';

@Module({
  providers: [LuckyDraftService]
})
export class LuckyDraftModule {}
