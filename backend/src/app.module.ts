import { Module } from '@nestjs/common';
import { AppController } from './app.controller';
import { AppService } from './app.service';
import { TypeOrmModule } from '@nestjs/typeorm';
import { ConfigModule, ConfigService } from '@nestjs/config';

@Module({
  imports: [
    TypeOrmModule.forRootAsync({
      inject: [ConfigService],
      imports: [ConfigModule.forRoot({isGlobal: true})],
      useFactory: (config: ConfigService) => {
        const DATABASE_HOST = config.get("DATABASE_HOST");
        const DATABASE_PORT = config.get("DATABASE_PORT");
        const DATABASE_NAME = config.get("DATABASE_NAME");
        const DATABASE_USER = config.get("DATABASE_USER");
        const DATABASE_PASSWORD = config.get("DATABASE_PASSWORD");
         return {
           type: "mysql",
           host: DATABASE_HOST,
           port: DATABASE_PORT,
           database: DATABASE_NAME,
           username: DATABASE_USER,
           password: DATABASE_PASSWORD,
         }
      }
    })
  ],
  controllers: [AppController],
  providers: [AppService],
})
export class AppModule {}
