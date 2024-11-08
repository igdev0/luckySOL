import { IsString, MaxLength } from 'class-validator';

export class UpdateTicketDto {
  @IsString()
  @MaxLength(32)
  transaction_id: string;
}
