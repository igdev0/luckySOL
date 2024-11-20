import { Module } from '@nestjs/common';
import { LuckyDraftService } from './lucky-draft.service';
import { TypeOrmModule } from '@nestjs/typeorm';
import { Ticket } from '../ticket/entities/ticket.entity';
import { LuckyDraftController } from './lucky-draft.controller';
import { LuckyDraftEntity } from './entities/lucky-draft.entity';

@Module({
  imports: [TypeOrmModule.forFeature([Ticket, LuckyDraftEntity])],
  providers: [LuckyDraftService],
  controllers: [LuckyDraftController],
})
export class LuckyDraftModule {}
