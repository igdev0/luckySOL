import {PublicKey, SYSVAR_RENT_PUBKEY, TransactionInstruction} from '@solana/web3.js';
import {serialize} from '@dao-xyz/borsh';
import {PROGRAM_ID, SYSTEM_PROGRAM_ID, TOKEN_PROGRAM_ID} from './constants';
import {
  findPlayerAccountPDA,
  findPlayerTokenAccountPDA,
  findPoolStoragePDA,
  findReceiptPoolMintPDA
} from './helpers';
import {
  ClosePlayerAccount,
  Deposit,
  DraftWinner,
  InitializePool, PlayerWithdraw,
  PurchaseTicket,
  SelectWinnersAndAirdrop,
  TicketAccountData
} from './instructions';

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
  });
}

export function processPurchaseTicketInstruction(ticketAccountData: TicketAccountData, poolAuthority: PublicKey, player: PublicKey) {
  const data = Buffer.from(serialize(new PurchaseTicket(ticketAccountData)));
  const [poolPDA] = findPoolStoragePDA(poolAuthority);
  const [mintPDA] = findReceiptPoolMintPDA(poolAuthority);
  const [playerPDA] = findPlayerAccountPDA(player);
  const [playerTokenPDA] = findPlayerTokenAccountPDA(player);

  return new TransactionInstruction({
    data,
    programId: PROGRAM_ID,
    keys: [
      // 1. Pool authority
      {
        pubkey: poolAuthority,
        isSigner: false,
        isWritable: true,
      },
      // 2. Account payer
      {
        pubkey: player,
        isSigner: true,
        isWritable: true,
      },
      // 3. Account PDA for payer
      {
        pubkey: playerPDA,
        isSigner: false,
        isWritable: true,
      },
      // 4. The player token account
      {
        pubkey: playerTokenPDA,
        isSigner: false,
        isWritable: true,
      },
      // 5. Stake pool vault
      {
        pubkey: poolPDA,
        isSigner: false,
        isWritable: true,
      },
      // 6. Stake pool mint account
      {
        pubkey: mintPDA,
        isSigner: false,
        isWritable: true,
      },
      // 7. Rent account
      {
        pubkey: SYSVAR_RENT_PUBKEY,
        isSigner: false,
        isWritable: false,
      },
      // 8. SPL 2022 account
      {
        pubkey: TOKEN_PROGRAM_ID,
        isSigner: false,
        isWritable: false,
      },
      // 9. System account
      {
        pubkey: SYSTEM_PROGRAM_ID,
        isSigner: false,
        isWritable: false,
      }
    ]
  });
}

export function processDraftWinners(poolAuthority: PublicKey, winners: DraftWinner[]) {
  const data = Buffer.from(serialize(new SelectWinnersAndAirdrop(winners)));
  const [poolStoragePDA] = findPoolStoragePDA(poolAuthority);
  const [poolMintPDA] = findReceiptPoolMintPDA(poolAuthority);
  const keys = winners.flatMap(winner => ([
    {
      pubkey: new PublicKey(winner.address),
      isWritable: true,
      isSigner: false
    },
    {
      pubkey: new PublicKey(winner.token_account),
      isSigner: false,
      isWritable: true
    }
  ]));
  return new TransactionInstruction({
    data,
    programId: PROGRAM_ID,
    keys: [
      {
        pubkey: poolAuthority,
        isSigner: true,
        isWritable: false,
      },
      {
        pubkey: poolStoragePDA,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: poolMintPDA,
        isSigner: false,
        isWritable: true,
      },
      {
      pubkey: TOKEN_PROGRAM_ID,
        isSigner: false,
        isWritable: false,
      }, ...keys]
  });
}

export function processPlayerWithdraw(amount: bigint, player: PublicKey) {
  const [playerPDA] = findPlayerAccountPDA(player);
  const data = Buffer.from(serialize(new PlayerWithdraw(amount)));
  return new TransactionInstruction({
    data,
    programId: PROGRAM_ID,
    keys: [
      {
        pubkey: player,
        isSigner: true,
        isWritable: true,
      },
      {
        pubkey: playerPDA,
        isSigner: false,
        isWritable:true
      }
    ]
  })
}

export function processClosePlayerAccountInstruction(poolAuthority: PublicKey, player: PublicKey) {
  const data = Buffer.from(serialize(new ClosePlayerAccount()));
  const [poolMintPDA] = findReceiptPoolMintPDA(poolAuthority);
  const [playerPDA] = findPlayerAccountPDA(player);
  const [playerTokenPDA] = findPlayerTokenAccountPDA(player);

  return new TransactionInstruction({
    data,
    programId: PROGRAM_ID,
    keys: [
      {
        pubkey: player,
        isSigner: true,
        isWritable: true,
      },
      {
        pubkey: playerPDA,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: playerTokenPDA,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: poolAuthority,
        isSigner: false,
        isWritable: true
      },
      {
        pubkey: poolMintPDA,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: TOKEN_PROGRAM_ID,
        isSigner: false,
        isWritable: false
      }
    ],
  })
}