import {
  Column,
  CreateDateColumn,
  Entity,
  PrimaryGeneratedColumn,
  UpdateDateColumn,
} from 'typeorm';

export type TicketStatus = 'PendingCreation' | 'Created' | 'Win' | 'Loss';

@Entity()
export class Ticket {
  @PrimaryGeneratedColumn('uuid')
  id: string;

  @Column({
    type: 'json',
    nullable: false,
  })
  lucky_draft: JSON;

  @Column({
    type: 'text',
  })
  status: 'PendingCreation' | 'Created' | 'Win' | 'Loss';

  @Column({
    nullable: true,
    default: null,
  })
  transaction_id: string;

  @Column({
    nullable: false,
  })
  merkle_hash: string;

  @Column({
    nullable: false,
  })
  address: string;

  @CreateDateColumn()
  created_at: string;

  @UpdateDateColumn()
  updated_at: string;
}
