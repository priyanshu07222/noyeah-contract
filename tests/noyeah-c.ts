import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import {NoyeahC} from "../target/types/noyeah_c"
import {Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram} from "@solana/web3.js"
import { assert } from "chai";

describe("no-yeah_c", () => {
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)
  const wallet = provider.wallet as anchor.Wallet
  const program = anchor.workspace.NoyeahC as Program<NoyeahC>
  
  let creatorKeypair;
  let participantKeypair;
  let contestPda;
  let contestVaultPda;
  let contestTitle: string;
  let entryFee;
  let endTime;
  
  beforeEach(async()=>{
    console.log("before each start")
    creatorKeypair = new Keypair()
    participantKeypair = anchor.web3.Keypair.generate()
    contestTitle = "Check this"
    entryFee = new anchor.BN(1_000_000);
    endTime = new anchor.BN(1846657334);
    console.log("is before each reach here");
    console.log(program.programId, "yeshh");
    console.log(contestTitle);
    console.log(creatorKeypair.publicKey);
    [contestPda] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("contest"), Buffer.from(contestTitle)], program.programId);
    [contestVaultPda] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("vault"), contestPda.toBuffer()], program.programId);
    console.log("after pda");
    
    const airdropSignature = await provider.connection.requestAirdrop(creatorKeypair.publicKey, 1 * LAMPORTS_PER_SOL)
    const sig = await provider.connection.confirmTransaction(airdropSignature);
    console.log(sig, airdropSignature)
    const balance = await provider.connection.getBalance(creatorKeypair.publicKey)
    
    const airdropSignatureParticipant = await provider.connection.requestAirdrop(participantKeypair.publicKey, 1 * LAMPORTS_PER_SOL)
    const sigParticipant = await provider.connection.confirmTransaction(airdropSignature);
    
    console.log(balance)
    console.log(await provider.connection.getBalance(participantKeypair.publicKey))
  })
  
  it("create-contest", async() => {
    console.log("reached here");
    const transactionSig = await program.methods
      .createContest(contestTitle, endTime, entryFee)
      .accounts({
        signer: creatorKeypair.publicKey,
        createContest: contestPda,
        contestVaultAccount: contestVaultPda,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .signers([creatorKeypair])
      .rpc()
    
    console.log(transactionSig)
    
    const contestAccount = await program.account.createContestState.fetch(
          contestPda
    );
    
    console.log("lets goo guys")
    console.log(contestAccount.startTime.toString(), contestAccount.endTime.toString(), contestAccount.creator.toString(), contestAccount.title, contestAccount.status.open, contestAccount.yesEntryPrice.toNumber())
    
    assert.equal(contestAccount.creator.toString(), creatorKeypair.publicKey.toString());
    assert.equal(contestAccount.title, contestTitle);
    assert.equal(contestAccount.yesEntryPrice.toNumber(), entryFee);
    assert.equal(contestAccount.noEntryPrice.toNumber(), entryFee);
  })
  
  // it("hello", async () => {
  //   console.log("delhi")
  //     const [participantPda] = await anchor.web3.PublicKey.
  //       findProgramAddress(
  //         [Buffer.from("bid"), 
  //           creatorKeypair.publicKey.toBuffer(), 
  //           contestPda.toBuffer()
  //           ],
  //         program.programId
  //       )
  //     console.log("meow")
  //   console.log(participantKeypair.publicKey);
  //   await provider.connection.requestAirdrop(participantKeypair.publicKey, 2 * LAMPORTS_PER_SOL);
  //   setTimeout(()=> {console.log("wait 1 min")}, 1000)
  //   const crebal = await provider.connection.getBalance(participantKeypair.publicKey);
    
  //   console.log(crebal)
  //   const hellosig = await program.methods.hello().
  //     accounts({
  //       payer: creatorKeypair.publicKey,
  //       participantAccount: participantPda,
  //       contestAccount: contestPda,
  //       contestVault: contestVaultPda,
  //       systemProgram: anchor.web3.SystemProgram.programId
  //     }).signers([creatorKeypair]).rpc()
  // })
  
  it("participate in contest", async () => {
    // const transactionSig = await program.methods
    //   .createContest(contestTitle, endTime, entryFee)
    //   .accounts({
    //     signer: creatorKeypair.publicKey,
    //     createContest: contestPda,
    //     contestVaultAccount: contestVaultPda,
    //     systemProgram: anchor.web3.SystemProgram.programId
    //   })
    //   .signers([creatorKeypair])
    //   .rpc()
    console.log("delhi")
    const [participantPda] = await anchor.web3.PublicKey.
      findProgramAddress(
        [Buffer.from("bid"), 
          creatorKeypair.publicKey.toBuffer(), 
          contestPda.toBuffer()],
        program.programId
      )
    console.log("participant pda -> ", participantPda.toBase58())
    console.log("createContest pda", contestPda.toBase58())
    // console.log(transactionSig.toString())
    console.log("right now war season")
    try {
      const participateSig = await program.
        methods.
        participateContest(entryFee, { yes: {} }).
        accounts({
          payer: creatorKeypair.publicKey,
          participantAccount: participantPda,
          contestAccount: contestPda,
          contestVault: contestVaultPda,
          systemProgram: anchor.web3.SystemProgram.programId
        }).
        signers([creatorKeypair]).
        rpc();
      
    } catch(err) {
      const logs = (err as any).logs;
        if (logs) {
          console.error("Logs:", logs.join("\n"));
        }
        throw err;
    }
  
   console.log("but army is there")
  
  const participantAccount = await program.account.participantState.fetch(
        participantPda
      );
  
    const contestAccount1 = await program.account.createContestState.fetch(contestPda);
  
    console.log(contestAccount1.yesEntryPrice.toNumber(), "checking yes entry price");
  

  
  console.log("done")
  
  assert.equal(participantAccount.participant.toString(), creatorKeypair.publicKey.toString());
  assert.equal(participantAccount.contest.toString(), contestPda.toString());
  assert.equal(participantAccount.amount.toNumber(), entryFee);
  assert.equal(participantAccount.isWinner, false); 
  
  console.log(participantAccount.isWinner, participantAccount.contest.toBase58(), participantAccount.amount, participantAccount.participant, participantAccount.option)
  console.log(await provider.connection.getBalance(contestVaultPda), "hello")
  })
})