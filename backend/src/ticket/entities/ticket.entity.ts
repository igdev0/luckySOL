import { Column, CreateDateColumn, Entity, PrimaryGeneratedColumn, UpdateDateColumn } from 'typeorm';

@Entity()
export class Ticket {
  @PrimaryGeneratedColumn("uuid")
  id: string

  @Column({
    type: "json",
    nullable: false,
  })
  lucky_draft: JSON

  @Column({
    type: "enum",
    enum: ['PendingCreation', 'Created', 'Win', 'Loss']
  })
  status: 'PendingCreation' | 'Created' | 'Win' | 'Loss'

  @Column({
    nullable: false,
  })
  merkle_hash: string

  @Column({
    nullable: false
  })
  address: string

  @CreateDateColumn()
  created_at: string

  @UpdateDateColumn()
  updated_at: string
}
