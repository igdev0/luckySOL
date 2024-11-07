import { IsJSON, IsString, MaxLength, MinLength } from 'class-validator';

export class CreateTicketDto {
  @MaxLength(44)
  @MinLength(32)
  @IsString()
  address: string
  @IsJSON({message: "This field has to be JSON"})
  lucky_draft: string
}
