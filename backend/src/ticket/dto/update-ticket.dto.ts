import { PartialType } from '@nestjs/mapped-types';
import { CreateTicketDto } from './create-ticket.dto';
import { IsString, MaxLength, MinLength } from 'class-validator';

export class UpdateTicketDto {
  @IsString()
  @MaxLength(32)
  transaction_id: string

  @MaxLength(44)
  @MinLength(32)
  @IsString()
  address: string
}
