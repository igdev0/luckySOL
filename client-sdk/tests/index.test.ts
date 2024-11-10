import {
  Connection,
  Keypair,
  sendAndConfirmTransaction,
  Transaction,
} from '@solana/web3.js';
import * as path from 'path';
import {InitializePool, PoolStorageData} from '../dist/instructions.js';
import * as fs from 'node:fs';
import {processPoolInitializationInstruction} from '../dist/index.js';

const payer_path = path.join(__dirname, '../program/solana_lottery_program-keypair.json');

const payer_buffer = fs.readFileSync(payer_path, "utf-8");
const parsed = Uint8Array.from(JSON.parse(payer_buffer));
const payer = Keypair.fromSecretKey(parsed);
// @todo:
// - implement before and after setup
// beforeAll(async () => {
//   // Start Solana validator
//   validatorProcess = child_process.spawn('solana-test-validator', [], { stdio: 'inherit' });
//   // expect(payer.publicKey.toString()).toEqual("6LQLQ6Xd8D8DdjAuyV9PzyhKZHsXwaN6zEwDVZLPhgPh")
//
//   // Wait for the validator to start
//   await new Promise((resolve) => setTimeout(resolve, 3000)); // adjust delay as needed
//   //
//   // Set up a connection to the local validator
//   const connection = new Connection('http://localhost:8899', 'confirmed');
//
//   const program = await connection.getAccountInfo(payer.publicKey);
//
//   if(program) {
//     console.log("The program is deployed already")
//     return null;
//   }
//
//   // Airdrop SOL to a Keypair to use for deploying the program
//   // AcibwgJE5WrCXh5Yatwzwfmaz5RuKQ9Z2ktTsq1hP8y7
//   expect(payer).not.toBeUndefined();
//   await connection.requestAirdrop(payer.publicKey, 100 * LAMPORTS_PER_SOL);
//
//   async function waitForAccountChange() {
//
//     const gen = await connection.getBalance(payer.publicKey)
//     return new Promise(resolve => {
//       connection.onAccountChange(payer.publicKey, (event) => {
//         console.log(event.lamports, ' +> total lamports after the change');
//         // Compile and deploy the custom program
//         const programPath = path.join(__dirname, '../program/solana_lottery_program.so'); // Update the path
//         // const programId = Keypair.generate(); // Generate a new keypair for the program
//
//         child_process.execSync("solana config set --url http://localhost:8899");
//         // Deploy the program (adjust command if using Anchor or a specific toolchain)
//         child_process.execSync(`solana program deploy ${programPath} --program-id ${payer_path}`);
//
//         resolve(event);
//       })
//     })
//   }
//
//   await waitForAccountChange();
//
//
// });

describe("Program main features", () => {
    it("Should initialize pool storage", async () => {
      const connection = new Connection('http://localhost:8899', 'confirmed');
      const poolData = new PoolStorageData({
        initial_amount: BigInt(10),
        draft_count: BigInt(0),
        ticket_price: BigInt(50)
      });

      const poolInitialization = new InitializePool(poolData);
      const {blockhash} = await connection.getLatestBlockhash();

      const txInstruction = processPoolInitializationInstruction(poolInitialization, payer.publicKey);

      const tx = new Transaction({recentBlockhash: blockhash, feePayer: payer.publicKey}).add(txInstruction);
      await sendAndConfirmTransaction(connection, tx, [payer], {commitment: "confirmed"})
      expect(1 + 1).toEqual(2);
    })
})