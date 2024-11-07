import { Inject, Injectable } from '@nestjs/common';
import { CreateTicketDto } from './dto/create-ticket.dto';
import { UpdateTicketDto } from './dto/update-ticket.dto';
import { InjectRepository, TypeOrmModule } from '@nestjs/typeorm';
import { Ticket } from './entities/ticket.entity';
import { Repository } from 'typeorm';

@Injectable()
export class TicketService {
  constructor(@InjectRepository(Ticket) private ticketsRepository: Repository<Ticket>) {
  }
  async create(createTicketDto: CreateTicketDto) {
    const entity = this.ticketsRepository.create({
      address: createTicketDto.address,
      lucky_draft: createTicketDto.lucky_draft,
      merkle_hash: "Tobe implemented",
      status: "PendingCreation",
    })

    return await this.ticketsRepository.save(entity);
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
