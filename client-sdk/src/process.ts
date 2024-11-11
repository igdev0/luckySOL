import {InitializePool} from '../dist/instructions.js';
import {PublicKey, SYSVAR_RENT_PUBKEY, TransactionInstruction} from '@solana/web3.js';
import {serialize} from '@dao-xyz/borsh';
import {PROGRAM_ID, SYSTEM_PROGRAM_ID, TOKEN_PROGRAM_ID} from './constants.js';
import {findPoolStoragePDA, findReceiptPoolMintPDA} from './helpers.js';
import {Deposit} from './instructions';

export function processPoolInitializationInstruction(data: InitializePool, payer: PublicKey) {
  const dataSerialized = Buffer.from(serialize(data));
  const [poolStoragePDA] = findPoolStoragePDA(payer);
  const [mintPDA] = findReceiptPoolMintPDA(payer);

  return new TransactionInstruction({
    data: dataSerialized,
    programId: PROGRAM_ID,
    keys: [
      {
        pubkey: payer,
        isSigner: true,
        isWritable: false
      },
      {
        pubkey: poolStoragePDA,
        isWritable: true,
        isSigner: false
      },
      {
        pubkey: mintPDA,
        isWritable: true,
        isSigner: false,
      },
      {
        pubkey: SYSVAR_RENT_PUBKEY,
        isSigner: false,
        isWritable: false
      },
      {
        pubkey: TOKEN_PROGRAM_ID,
        isWritable: false,
        isSigner: false
      },
      {
        pubkey: SYSTEM_PROGRAM_ID,
        isSigner: false,
        isWritable: false,
      }
    ]
  });

}


export function processDepositInstruction(amount: bigint, payer: PublicKey, pool_authority: PublicKey) {
  const deposit = new Deposit(amount);
  const data = Buffer.from(serialize(deposit));
  const [poolPDA] = findPoolStoragePDA(pool_authority);
    return new TransactionInstruction({
      data,
      programId: PROGRAM_ID,
      keys: [
        {
          pubkey: payer,
          isSigner: true,
          isWritable: false,
        },
        {
          pubkey: poolPDA,
          isSigner: false,
          isWritable: true,
        },
        {
          pubkey: SYSTEM_PROGRAM_ID,
          isSigner: false,
          isWritable: false,
        }
      ]
    })
}