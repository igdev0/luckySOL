import { Module } from '@nestjs/common';
import { TypeOrmModule } from '@nestjs/typeorm';
import { Ticket } from '../ticket/entities/ticket.entity';

@Module({
  imports: [
    TypeOrmModule.forRoot({
      type: 'sqlite',
      database: ':memory:',
      entities: [Ticket],
      synchronize: true,
      dropSchema: true,
    }),
  ],
  exports: [],
})
export class DatabaseTestModule {}
