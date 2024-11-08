import { Test, TestingModule } from '@nestjs/testing';
import { TicketService } from './ticket.service';
import { TypeOrmModule } from '@nestjs/typeorm';
import { Ticket } from './entities/ticket.entity';
import { DatabaseTestModule } from '../database-test/database-test.module';

describe('TicketService', () => {
  let service: TicketService;

  beforeEach(async () => {
    const module: TestingModule = await Test.createTestingModule({
      imports: [DatabaseTestModule, TypeOrmModule.forFeature([Ticket])],
      providers: [TicketService],
    }).compile();

    service = module.get<TicketService>(TicketService);
  });

  it('should be defined', () => {
    expect(service).toBeDefined();
  });
});
