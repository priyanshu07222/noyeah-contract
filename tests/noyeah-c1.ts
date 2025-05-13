// import * as anchor from "@coral-xyz/anchor";
// import { Program } from "@coral-xyz/anchor";
// import { NoyeahC } from "../target/types/noyeah_c";
// import path from "path"
// import * as borsh from "borsh"
// import { sha256 } from "js-sha256";

// import { LiteSVM } from "litesvm";
// import {
//   Transaction,
//   PublicKey,
//   SystemProgram,
//   Keypair,
//   LAMPORTS_PER_SOL,
//   TransactionInstruction,
// } from "@solana/web3.js";
// import { expect } from "chai";
// import { title } from "process";

// class CreateContestArgs{
//   title: string
//   end_time: anchor.BN
//   entry_fee: anchor.BN
  
//   constructor(fields: { title: string; end_time: anchor.BN; entry_fee: anchor.BN }) {
//       this.title = fields.title;
//       this.end_time = fields.end_time;
//       this.entry_fee = fields.entry_fee;
//     }
// }


// const CreateContestSchema = new Map([
//   [
//     CreateContestArgs,
//     {
//       kind: "struct",
//       fields: [
//         ["title", "string"],
//         ["end_time", "i64"],
//         ["entry_fee", "u64"],
//       ],
//     },
//   ],
// ]);

// const schema: borsh.Schema = {
//   struct: {
//     title: 'string',
//     end_time: 'i64',
//     entry_fee: 'u64'
//   }
// }

// function getDiscriminator(name: string): Buffer {
//   const preimage = `global:${name}`;
//   const hash = sha256.digest(preimage);
//   return Buffer.from(hash).slice(0, 8);
// }

// describe("noyeah-c", () => {
//   const svm = new LiteSVM;
//   const programId = PublicKey.unique();
//   let payer:Keypair;
//   let programPath:string;
//   let contestPda;
//   let contestVaultPda;
//   let contestTitle: string;

  
  
//   beforeEach(async () => {
//     payer = new Keypair();
//     contestTitle = "Check create contest"
//     programPath = path.resolve(__dirname, "../target/deploy/noyeah_c.so");
    
//     svm.airdrop(payer.publicKey, BigInt(LAMPORTS_PER_SOL));
//     svm.addProgramFromFile(programId, programPath);
    
//     console.log(payer.publicKey, "hello")
    
//     contestPda = await PublicKey.findProgramAddressSync([Buffer.from("contest"), Buffer.from(contestTitle)], programId)
//     contestVaultPda = await PublicKey.findProgramAddressSync([Buffer.from("vault"), contestPda[0].toBuffer()], programId)
    
//     console.log(svm.getBalance(payer.publicKey))
//   })
  
//   it("create contest", async()=> {
//     const blockhash = svm.latestBlockhash();
//     const endTime = 1846657334
//     const entry_fee = 1_000_000;
//     console.log("are u working my bro")
    
//     const args = new CreateContestArgs({
//       title: contestTitle,
//       end_time: new anchor.BN(endTime),
//       entry_fee: new anchor.BN(entry_fee)
//     })
    
//     console.log("fuck you ")
    
//     const serialiseArgs = borsh.serialize(schema, args);
//     const discriminator = getDiscriminator(title)
//     const instructionData = Buffer.concat([discriminator, Buffer.from(serialiseArgs)])
   
    
//     console.log("ganduu hai kya")
    
//     const ixs = new TransactionInstruction({
//         programId: programId,
//         keys: [
//           {pubkey: payer.publicKey, isSigner: true, isWritable: true},
//           {pubkey: contestPda, isSigner: false, isWritable: true},
//           {pubkey: contestVaultPda, isSigner:false, isWritable: false},
//           // {pubkey: SystemProgram.programId, isSigner:false, isWritable: false}
//         ],
//         data: instructionData
//       })
    
//     console.log("let's see")
    
    
//     const tx = new Transaction().add(ixs)
//     console.log("seems like problem is here")
//     tx.recentBlockhash = blockhash
    
//     tx.feePayer = payer.publicKey;
//     console.log("or maybe after this")
//     tx.sign(payer)
    
//     console.log("reaching here or not")
    
//     const simRes = svm.simulateTransaction(tx);
//     const sendRes = svm.sendTransaction(tx)
//     // const sendRes = svm.sendTransaction(tx)
//     expect(sendRes).to.be.ok;
    
//     const contestAccount = svm.getAccount(contestPda);
//     expect(contestAccount).to.be.exist;
//   })

//   // anchor.setProvider(anchor.AnchorProvider.env());

//   // const program = anchor.workspace.noyeahC as Program<NoyeahC>;

//   // it("Is contest initialized!", async () => {
//   //   // Add your test here.

//   //   const tx = await svm.methods.createContest();
//   //   console.log("Your transaction signature", tx);
//   // });
// });

// // class CreateContestArgs{
// //   title: string
// //   end_time: anchor.BN
// //   entry_fee: anchor.BN
  
// //   constructor(fields: { title: string; end_time: BN; entry_fee: BN }) {
// //       this.title = fields.title;
// //       this.end_time = fields.end_time;
// //       this.entry_fee = fields.entry_fee;
// //     }
// // }


// // const CreateContestSchema = new Map([
// //   [
// //     CreateContestArgs,
// //     {
// //       kind: "struct",
// //       fields: [
// //         ["title", "string"],
// //         ["end_time", "u64"],
// //         ["entry_fee", "u64"],
// //       ],
// //     },
// //   ],
// // ]);