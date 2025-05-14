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
  // let participantPda;
  
  creatorKeypair = new Keypair()
  beforeEach(async()=>{
    participantKeypair = anchor.web3.Keypair.generate()
    contestTitle = "Check this"
    entryFee = new anchor.BN(1_000_000);
    endTime = new anchor.BN(1847208378);
    [contestPda] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("contest"), Buffer.from(contestTitle)], program.programId);
    [contestVaultPda] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("vault"), contestPda.toBuffer()], program.programId);

    
    const airdropSignature = await provider.connection.requestAirdrop(creatorKeypair.publicKey, 1 * LAMPORTS_PER_SOL)
    const sig = await provider.connection.confirmTransaction(airdropSignature);
    const balance = await provider.connection.getBalance(creatorKeypair.publicKey)
    
    const airdropSignatureParticipant = await provider.connection.requestAirdrop(participantKeypair.publicKey, 1 * LAMPORTS_PER_SOL)
    const sigParticipant = await provider.connection.confirmTransaction(airdropSignature);
    
  })
  
  it("create-contest", async() => {
    console.log("reached here", creatorKeypair.publicKey.toBase58());
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
    
    
    const contestAccount = await program.account.createContestState.fetch(
          contestPda
    );
    // console.log(contestAccount.startTime.toString(), contestAccount.endTime.toString(), contestAccount.creator.toString(), contestAccount.title, contestAccount.status.open, contestAccount.yesEntryPrice.toNumber())
    
    assert.equal(contestAccount.creator.toString(), creatorKeypair.publicKey.toString());
    assert.equal(contestAccount.title, contestTitle);
    assert.equal(contestAccount.yesEntryPrice.toNumber(), entryFee);
    assert.equal(contestAccount.noEntryPrice.toNumber(), entryFee);
  })
  
  
  it("participate in contest", async () => {
    const [participantPda] = await anchor.web3.PublicKey.
      findProgramAddress(
        [Buffer.from("bid"), 
          creatorKeypair.publicKey.toBuffer(), 
          contestPda.toBuffer()],
        program.programId
      )
    // console.log("participant pda -> ", participantPda.toBase58())
    // console.log("createContest pda", contestPda.toBase58())
    // console.log(transactionSig.toString())
    // console.log("right now war season")
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
  
  
  const participantAccount = await program.account.participantState.fetch(
        participantPda
      );
  
    const contestAccount1 = await program.account.createContestState.fetch(contestPda);
  
    // console.log(contestAccount1.yesEntryPrice.toNumber(), "checking yes entry price");
  
  assert.equal(participantAccount.participant.toString(), creatorKeypair.publicKey.toString());
  assert.equal(participantAccount.contest.toString(), contestPda.toString());
  assert.equal(participantAccount.amount.toNumber(), entryFee);
  assert.equal(participantAccount.isWinner, false); 
  
  // console.log(participantAccount.isWinner, participantAccount.contest.toBase58(), participantAccount.amount, participantAccount.participant, participantAccount.option)
  // console.log(await provider.connection.getBalance(contestVaultPda), "hello")
  })
  
  it("resolve contest", async() =>{
    const contestAccount1 = await program.account.createContestState.fetch(contestPda);
    const resolveTxn = await program.methods
      .resolveContest({yes: {}})
      .accounts({
        payer: creatorKeypair.publicKey,
        contest: contestPda
      })
      .signers([creatorKeypair])
      .rpc()
    
    const contestAccount = await program.account.createContestState.fetch(contestPda);
    // console.log("lets see what it's printing", contestAccount.status, contestAccount.correctAnswer, contestAccount.winnerCount.toNumber());
    
    assert.deepEqual(contestAccount.status, { resolved: {} });
    assert.deepEqual(contestAccount.correctAnswer, { yes: {} });
  })
  
  it("finalize_contest", async() => {
    
    const [participantPda] = await anchor.web3.PublicKey.
      findProgramAddress(
        [Buffer.from("bid"), 
          creatorKeypair.publicKey.toBuffer(), 
          contestPda.toBuffer()],
        program.programId
      )

    const finalize_contest = await program
      .methods
      .finalizeContest()
      .accounts({
        payer: creatorKeypair.publicKey,
        contestAccount: contestPda,
        contestVault: contestVaultPda,
        participant: participantPda,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .signers([creatorKeypair])
      .rpc()
    
    
    const contestAccount = await program.account.createContestState.fetch(contestPda);
    const participant = await program.account.participantState.fetch(participantPda);
    // console.log(participant.isWinner, contestAccount.status, contestAccount.winnerCount.toNumber(), participant.hasClaimed)
    assert.deepEqual(contestAccount.status, {resolved: {}})
    assert.equal(participant.isWinner, true)
  })
})