import { Controller, Get, Inject, Query } from '@nestjs/common';
import { LuckyDraftService } from './lucky-draft.service';

@Controller('lucky-draft')
export class LuckyDraftController {
  constructor(@Inject(LuckyDraftService) private service: LuckyDraftService) {}

  @Get()
  get(@Query('page') page = 1, @Query('limit') limit = 10) {
    return this.service.findAll(page, limit);
  }
}
