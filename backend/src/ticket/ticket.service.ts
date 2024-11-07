import { Inject, Injectable } from '@nestjs/common';
import { CreateTicketDto } from './dto/create-ticket.dto';
import { UpdateTicketDto } from './dto/update-ticket.dto';
import { InjectRepository, TypeOrmModule } from '@nestjs/typeorm';
import { Ticket } from './entities/ticket.entity';
import { Repository } from 'typeorm';
import * as crypto from 'node:crypto';
@Injectable()
export class TicketService {
  constructor(@InjectRepository(Ticket) private ticketsRepository: Repository<Ticket>) {
  }
  async create(createTicketDto: CreateTicketDto) {
    const entity = this.ticketsRepository.create({
      address: createTicketDto.address,
      lucky_draft: createTicketDto.lucky_draft,
      merkle_hash: this.hashTicket(JSON.stringify(createTicketDto.lucky_draft)),
      status: "PendingCreation",
    })

    return await this.ticketsRepository.save(entity);
  }

  private hashTicket(lucky_draft: string) {
    return crypto.hash("sha256", lucky_draft)
  }

  findAll() {
    return `This action returns all ticket`;
  }

  findOne(id: number) {
    return `This action returns a #${id} ticket`;
  }

  update(id: number, updateTicketDto: UpdateTicketDto) {
    return `This action updates a #${id} ticket`;
  }

  remove(id: number) {
    return `This action removes a #${id} ticket`;
  }
}
