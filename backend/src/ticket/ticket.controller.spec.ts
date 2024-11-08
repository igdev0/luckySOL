import { Test, TestingModule } from '@nestjs/testing';
import { TicketController } from './ticket.controller';
import { TicketService } from './ticket.service';
import { DatabaseTestModule } from '../database-test/database-test.module';
import { TypeOrmModule } from '@nestjs/typeorm';
import { Ticket } from './entities/ticket.entity';

const DEMO_ADDRESS = 'FSvf6qv7oN4gr3DZPkpGe3TAJFbKLbCQLdrv827vGq3T';
function generateLuckyNumber() {
  return Math.random() * 66;
}

function randomLuckyDraft() {
  return [
    generateLuckyNumber(),
    generateLuckyNumber(),
    generateLuckyNumber(),
    generateLuckyNumber(),
    generateLuckyNumber(),
    generateLuckyNumber(),
  ];
}

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

  it('Should create ticket', async () => {
    const response = await controller.create({
      address: DEMO_ADDRESS,
      lucky_draft: [randomLuckyDraft()],
    });

    expect(response.merkle_root).toBeDefined();
    expect(controller).toBeDefined();
  });

  it('Should find a single ticket by ID', async () => {
    const creationResult = await controller.create({
      address: DEMO_ADDRESS,
      lucky_draft: [randomLuckyDraft()],
    });

    const response = await controller.findOne(creationResult.id);
    expect(response.id).toEqual(creationResult.id);
  });

  it('Should return paginated data', async () => {
    let i = 0;

    while (i < 50) {
      i += 1;
      await controller.create({
        address: DEMO_ADDRESS,
        lucky_draft: [randomLuckyDraft()],
      });
    }

    const response = await controller.findAll(1, 10);

    expect(response.data.length).toEqual(10);
    expect(response.total).toEqual(50);
  });
});
