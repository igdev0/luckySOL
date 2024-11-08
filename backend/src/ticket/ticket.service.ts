import { Injectable } from '@nestjs/common';
import { CreateTicketDto } from './dto/create-ticket.dto';
import { UpdateTicketDto } from './dto/update-ticket.dto';
import { InjectRepository } from '@nestjs/typeorm';
import { Ticket } from './entities/ticket.entity';
import { Repository } from 'typeorm';
import * as crypto from 'node:crypto';
import { MerkleTree } from 'merkletreejs';

type TicketCreationResult = {
  id: string;
  merkle_root: string;
};

@Injectable()
export class TicketService {
  constructor(
    @InjectRepository(Ticket) private ticketsRepository: Repository<Ticket>,
  ) {}
  async create(
    createTicketDto: CreateTicketDto,
  ): Promise<TicketCreationResult> {
    const entity = this.ticketsRepository.create({
      address: createTicketDto.address,
      lucky_draft: JSON.parse(JSON.stringify(createTicketDto.lucky_draft)),
      merkle_hash: this.hashTicket(JSON.stringify(createTicketDto.lucky_draft)),
      status: 'PendingCreation',
    });

    let { id } = await this.ticketsRepository.save(entity);
    const merkle_root = await this.getMerkleRoot(createTicketDto.address);
    return {
      id,
      merkle_root,
    };
  }

  async findAll(page: number, limit: number, address: string | null) {
    const [data, total] = await this.ticketsRepository.findAndCount({
      where: !address
        ? {}
        : {
            address: address,
          },
      skip: (page - 1) * limit,
      take: limit,
      order: { created_at: 'ASC' },
    });
    return {
      data,
      total,
      page,
      limit,
    };
  }

  findOne(id: string) {
    return this.ticketsRepository.findOne({
      where: { id },
    });
  }

  update(id: number, updateTicketDto: UpdateTicketDto) {
    return `This action updates a #${id} ticket`;
  }

  private async getMerkleRoot(address: string) {
    const tickets = await this.ticketsRepository.find({
      where: {
        address,
      },
      select: {
        merkle_hash: true,
      },
      order: {
        created_at: 'ASC',
      },
    });

    const tree = new MerkleTree(tickets.map((item) => item.merkle_hash));
    return tree.getRoot().toString('hex');
  }

  private hashTicket(lucky_draft: string) {
    return crypto.hash('sha256', lucky_draft);
  }
}
