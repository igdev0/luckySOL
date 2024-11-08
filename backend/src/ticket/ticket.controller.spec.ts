import { Test, TestingModule } from '@nestjs/testing';
import { TicketController } from './ticket.controller';
import { TicketService } from './ticket.service';
import { DatabaseTestModule } from '../database-test/database-test.module';
import { TypeOrmModule } from '@nestjs/typeorm';
import { Ticket } from './entities/ticket.entity';
import * as crypto from 'node:crypto';

const DEMO_ADDRESS_0 = 'FSvf6qv7oN4gr3DZPkpGe3TAJFbKLbCQLdrv827vGq3T';
const DEMO_ADDRESS_1 = 'FSvf6qv7oN4gr3DZPkpGe3TAJFbKLbCQLdrv827vGt3T';
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
      address: DEMO_ADDRESS_0,
      lucky_draft: [randomLuckyDraft()],
    });

    expect(response.merkle_root).toBeDefined();
    expect(controller).toBeDefined();
  });

  it('Should find a single ticket by ID', async () => {
    const creationResult = await controller.create({
      address: DEMO_ADDRESS_0,
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
        address: DEMO_ADDRESS_0,
        lucky_draft: [randomLuckyDraft()],
      });
    }

    const response = await controller.findAll(1, 10);

    expect(response.data.length).toEqual(10);
    expect(response.total).toEqual(50);
  });

  it('Should return paginated data for a given address', async () => {
    let i = 0;
    while (i < 30) {
      i += 1;
      await controller.create({
        address: DEMO_ADDRESS_1,
        lucky_draft: [randomLuckyDraft()],
      });
    }

    await controller.create({
      address: DEMO_ADDRESS_0,
      lucky_draft: [randomLuckyDraft()],
    });

    const response = await controller.findAll(1, 10, DEMO_ADDRESS_1);
    expect(response.total).toEqual(30);

    const allResponse = await controller.findAll(1, 10, null);
    expect(allResponse.total).toEqual(31);
  });

  it('Should update ticket transaction_id and its status', async () => {
    const ticket = await controller.create({
      address: DEMO_ADDRESS_0,
      lucky_draft: [randomLuckyDraft()],
    });

    expect(ticket.status).toEqual('PendingCreation');

    const transaction_id = crypto.hash('sha256', 'random');
    await controller.update(ticket.id, { transaction_id });

    const { transaction_id: tx_id, status } = await controller.findOne(
      ticket.id,
    );

    expect(status).toEqual('Created');
    expect(tx_id).toEqual(transaction_id);
  });
});
