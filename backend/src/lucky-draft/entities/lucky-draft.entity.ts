import {
  Column,
  CreateDateColumn,
  Entity,
  PrimaryGeneratedColumn,
  UpdateDateColumn,
} from 'typeorm';

@Entity('lucky-draft')
export class LuckyDraftEntity {
  @PrimaryGeneratedColumn('uuid')
  id: string;

  @Column({
    type: 'json',
    nullable: false,
  })
  lucky_draft: JSON;

  @Column({
    type: 'bigint',
    nullable: false,
  })
  total_prizes_won: BigInt;

  @CreateDateColumn()
  created_at: string;

  @UpdateDateColumn()
  updated_at: string;
}
