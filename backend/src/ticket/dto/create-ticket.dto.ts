import { ArrayContains, IsString, MaxLength, MinLength } from 'class-validator';

export class CreateTicketDto {
  @MaxLength(44)
  @MinLength(32)
  @IsString()
  address: string;
  @ArrayContains([Number])
  lucky_draft: [number[]];
}
