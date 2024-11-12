import {
  Connection,
  Keypair, LAMPORTS_PER_SOL,
  sendAndConfirmTransaction, SendTransactionError,
  Transaction, TransactionError,
} from '@solana/web3.js';
import * as path from 'path';
import {
  InitializePool,
  PoolStorageData,
  findPoolStoragePDA,
  processPoolInitializationInstruction,
  POOL_STORAGE_DATA_LENGTH,
  processDepositInstruction,
  TicketAccountData,
  processPurchaseTicketInstruction,
  findPlayerTokenAccountPDA, PurchaseTicket,
} from 'lucky-sol-sdk';
import * as fs from 'node:fs';
import * as child_process from 'node:child_process';
import {serialize} from '@dao-xyz/borsh';

const payer_path = path.join(__dirname, '../../program/target/deploy/solana_lottery_program-keypair.json');
const program_id_path = path.join(__dirname, '../program/1cky9mEdiuQ8wNCcw1Z7pXuxF9bsdxej95Gf69XydoA.json');

const payer_buffer = fs.readFileSync(payer_path, "utf-8");
const payer = Keypair.fromSecretKey(Uint8Array.from(JSON.parse(payer_buffer)));

let connection:Connection;
async function delay(ms = 1000) {
  return new Promise(resolve => {
    setTimeout(() => {
      resolve(null)
    }, ms)
  })
}

let validatorProcess: child_process.ChildProcess;
beforeAll(async () => {
  validatorProcess = child_process.spawn('solana-test-validator', ["--reset"], { stdio: 'inherit' });
  await new Promise((resolve) => setTimeout(resolve, 1000)); // adjust delay as needed
  // Set up a connection to the local validator
  connection = new Connection('http://localhost:8899', {
    commitment: "confirmed",
  });
  expect(payer).not.toBeUndefined();

  await connection.requestAirdrop(payer.publicKey, 100 * LAMPORTS_PER_SOL);
  await delay(1000)

  const programPath = path.join(__dirname, '../../program/target/deploy/solana_lottery_program.so'); // Update the path

  child_process.execSync("solana config set --url http://localhost:8899");
  child_process.execSync(`solana program deploy ${programPath} --program-id ${program_id_path} --fee-payer ${payer_path}`);
}, 8000);

afterAll(() => {
  validatorProcess.kill("SIGTERM");
}, 10000)

describe("Program main features", () => {
    it("Should initialize pool storage", async () => {
      // Connection.
      await delay(1000);
        const poolData = new PoolStorageData({
          initial_amount: (10 * LAMPORTS_PER_SOL).toString(),
          draft_count: (10 * LAMPORTS_PER_SOL).toString(),
          ticket_price: (.5 * LAMPORTS_PER_SOL).toString()
        });

        const poolInitialization = new InitializePool(poolData);
        const {blockhash} = await connection.getLatestBlockhash();

        const txInstruction = processPoolInitializationInstruction(poolInitialization, payer.publicKey);
        const tx = new Transaction({recentBlockhash: blockhash, feePayer: payer.publicKey}).add(txInstruction);
        await sendAndConfirmTransaction(connection, tx, [payer], {commitment: "confirmed"})

      const [poolPDA] = findPoolStoragePDA(payer.publicKey);
      const balance = await connection.getBalance(poolPDA, "confirmed");

      const rentExempt = await connection.getMinimumBalanceForRentExemption(POOL_STORAGE_DATA_LENGTH, "confirmed");

      expect(balance).toEqual(rentExempt + (10 * LAMPORTS_PER_SOL))
    })

  it('should deposit 10 SOL into the pool', async () => {
    const tx = new Transaction().add(processDepositInstruction(BigInt(10 * LAMPORTS_PER_SOL), payer.publicKey, payer.publicKey));
    await sendAndConfirmTransaction(connection, tx, [payer], {commitment: "confirmed"});

    const [poolPDA] = findPoolStoragePDA(payer.publicKey);
    const balance = await connection.getBalance(poolPDA, "confirmed");

    const rentExempt = await connection.getMinimumBalanceForRentExemption(POOL_STORAGE_DATA_LENGTH, "confirmed");
    expect(balance).toEqual(rentExempt + (20 * LAMPORTS_PER_SOL))
    // expect((balance - exemption) / LAMPORTS_PER_SOL).toEqual(10);
    // @todo:
    // - Figure out why the sendAndConfirmTransaction, is not aborting wss connection when specified a signal
    // await delay(1000);

  });


  it('should be able to purchase tickets', async () => {
    const player = Keypair.generate();
    await connection.requestAirdrop(player.publicKey, 50 * LAMPORTS_PER_SOL);
    // Wait until the airdrop completes
    await delay()
    const ticket = new Uint8Array([
      250, 35, 208, 216, 126, 242, 49, 214, 210, 183, 43, 30, 84, 173, 177, 56, 200, 187, 71,
      86, 115, 240, 96, 243, 152, 186, 199, 104, 179, 40, 50, 8]);
    const ticketAccountData = new TicketAccountData({total_tickets: BigInt(1), merkle_root: ticket});
    const instruction = processPurchaseTicketInstruction(ticketAccountData, payer.publicKey, player.publicKey);

    const tx = new Transaction().add(instruction);
    await sendAndConfirmTransaction(connection, tx, [player], {commitment: "confirmed"});

    const [playerTokenAccountPDA] = findPlayerTokenAccountPDA(player.publicKey);
    const playerTokenAccountInfo = await connection.getAccountInfo(playerTokenAccountPDA)
    expect(playerTokenAccountInfo?.data).not.toBeNull();
    // await delay()
  });

  it('should be able to serialize ticket account data', () => {
    const ticket = new Uint8Array([
      250, 35, 208, 216, 126, 242, 49, 214, 210, 183, 43, 30, 84, 173, 177, 56, 200, 187, 71,
      86, 115, 240, 96, 243, 152, 186, 199, 104, 179, 40, 50, 8]);

    const expectedBinaries = new Uint8Array([
      3, 250, 35, 208, 216, 126, 242, 49, 214, 210, 183, 43, 30, 84, 173, 177, 56, 200, 187, 71,
      86, 115, 240, 96, 243, 152, 186, 199, 104, 179, 40, 50, 8, 1, 0, 0, 0, 0, 0, 0, 0,
    ]);

      const data = new TicketAccountData({total_tickets: BigInt(1), merkle_root: ticket});
      const serializedInstr = new PurchaseTicket(data);
      const serialized = serialize(serializedInstr);
      expect(Array.from(serialized)).toEqual(Array.from(expectedBinaries));
  });
})