import { Test, TestingModule } from '@nestjs/testing';
import { TicketController } from './ticket.controller';
import { TicketService } from './ticket.service';
import { DatabaseTestModule } from '../database-test/database-test.module';
import { TypeOrmModule } from '@nestjs/typeorm';
import { Ticket } from './entities/ticket.entity';

describe('TicketController', () => {
  let controller: TicketController;

  beforeEach(async () => {
    const module: TestingModule = await Test.createTestingModule({
      imports: [DatabaseTestModule, TypeOrmModule.forFeature([Ticket])],
      controllers: [TicketController],
      providers: [TicketService],
    }).compile();

    controller = module.get<TicketController>(TicketController);
  });

  it('should be defined', () => {
    expect(controller).toBeDefined();
  });
});
