import { Module } from '@nestjs/common';
import { AppController } from './app.controller';
import { AppService } from './app.service';
import { TypeOrmModule } from '@nestjs/typeorm';
import { ConfigModule, ConfigService } from '@nestjs/config';
import { TicketModule } from './ticket/ticket.module';
import { Ticket } from './ticket/entities/ticket.entity';
import { DatabaseTestModule } from './database-test/database-test.module';
import { LuckyDraftModule } from './lucky-draft/lucky-draft.module';
import { ScheduleModule } from '@nestjs/schedule';
import { SolanaConnectionModule } from './solana-connection/solana-connection.module';
import { clusterApiUrl } from '@solana/web3.js';
import { LuckyDraftEntity } from './lucky-draft/entities/lucky-draft.entity';

@Module({
  imports: [
    TypeOrmModule.forRootAsync({
      inject: [ConfigService],
      imports: [
        ConfigModule.forRoot({ isGlobal: true }),
        ScheduleModule.forRoot({
          cronJobs: true,
        }),
      ],
      useFactory: (config: ConfigService) => {
        const DATABASE_HOST = config.get('DATABASE_HOST');
        const DATABASE_PORT = config.get('DATABASE_PORT');
        const DATABASE_NAME = config.get('DATABASE_NAME');
        const DATABASE_USER = config.get('DATABASE_USER');
        const DATABASE_PASSWORD = config.get('DATABASE_PASSWORD');
        return {
          type: 'mysql',
          host: DATABASE_HOST,
          port: DATABASE_PORT,
          database: DATABASE_NAME,
          username: DATABASE_USER,
          password: DATABASE_PASSWORD,
          synchronize: true,
          entities: [Ticket, LuckyDraftEntity],
        };
      },
    }),
    TicketModule,
    DatabaseTestModule,
    LuckyDraftModule,
    SolanaConnectionModule.forRootAsync({
      imports: [ConfigService],
      inject: [ConfigService],
      useFactory(config: ConfigService) {
        const endpoint =
          config?.get('RPC_URL') ?? clusterApiUrl('devnet', true);
        return {
          endpoint,
          commitmentOrConfig: 'confirmed',
        };
      },
    }),
  ],
  controllers: [AppController],
  providers: [AppService],
})
export class AppModule {}
