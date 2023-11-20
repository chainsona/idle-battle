import { log } from "console";

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Connection, PublicKey, TransactionSignature } from "@solana/web3.js";

import { IdleBattle } from "../target/types/idle_battle";

const BATTLE_SEED = "battle";
const HERO_SEED = "hero";

async function confirmAndLogTransaction(
  connection: Connection,
  txHash: TransactionSignature
) {
  const { blockhash, lastValidBlockHeight } =
    await connection.getLatestBlockhash();

  await connection.confirmTransaction(
    {
      blockhash,
      lastValidBlockHeight,
      signature: txHash,
    },
    "confirmed"
  );

  log(`Tx: https://explorer.solana.com/tx/${txHash}?cluster=custom`);
}

describe("Idle Battle", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const player = provider.wallet as anchor.Wallet;
  // const connection = new Connection("http://localhost:8899", "confirmed");

  const program = anchor.workspace.IdleBattle as Program<IdleBattle>;

  it("Hero is initialized", async () => {
    const [heroPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from(HERO_SEED), player.publicKey.toBuffer()],
      anchor.workspace.IdleBattle.programId
    );

    const tx = await program.methods
      .initialize()
      .accounts({
        player: player.publicKey,
        hero: heroPDA,
      })
      .rpc();
    await confirmAndLogTransaction(provider.connection, tx);
  });

  it("Hero stats are correct", async () => {
    const [heroPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from(HERO_SEED), player.publicKey.toBuffer()],
      anchor.workspace.IdleBattle.programId
    );

    const heroData = await program.account.hero.fetch(heroPDA);

    expect(heroData.trainingSlot.toNumber()).toBe(0);
    expect(heroData.xp.toNumber()).toBe(0);
    expect(heroData.level.toNumber()).toBe(0);
    expect(heroData.gold.toNumber()).toBe(0);
  });

  it("Hero is training", async () => {
    const [heroPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from(HERO_SEED), player.publicKey.toBuffer()],
      anchor.workspace.IdleBattle.programId
    );

    const heroDataBefore = await program.account.hero.fetch(heroPDA);

    const tx = await program.methods
      .train()
      .accounts({
        player: player.publicKey,
        hero: heroPDA,
      })
      .rpc();
    await confirmAndLogTransaction(provider.connection, tx);

    const heroDataAfter = await program.account.hero.fetch(heroPDA);

    expect(heroDataAfter.trainingSlot.toNumber()).toBeGreaterThan(0);
  });

  it("Hero answered to recall", async () => {
    const [heroPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from(HERO_SEED), player.publicKey.toBuffer()],
      anchor.workspace.IdleBattle.programId
    );

    const heroDataBefore = await program.account.hero.fetch(heroPDA);

    const txRecall = await program.methods
      .recall()
      .accounts({
        player: player.publicKey,
        hero: heroPDA,
      })
      .rpc();
    await confirmAndLogTransaction(provider.connection, txRecall);

    const heroDataAfter = await program.account.hero.fetch(heroPDA);

    expect(heroDataAfter.trainingSlot.toNumber()).toBe(0);
    expect(heroDataAfter.xp.toNumber()).toBeGreaterThan(
      heroDataBefore.xp.toNumber()
    );
  });

  it("Hero leveled up", async () => {
    const [heroPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from(HERO_SEED), player.publicKey.toBuffer()],
      anchor.workspace.IdleBattle.programId
    );

    const heroDataBefore = await program.account.hero.fetch(heroPDA);

    for (let i = 0; i < 2; i++) {
      const txTrain = await program.methods
        .train()
        .accounts({
          player: player.publicKey,
          hero: heroPDA,
        })
        .rpc();
      await confirmAndLogTransaction(provider.connection, txTrain);

      const txRecall = await program.methods
        .recall()
        .accounts({
          player: player.publicKey,
          hero: heroPDA,
        })
        .rpc();
      await confirmAndLogTransaction(provider.connection, txRecall);
    }

    const heroDataAfter = await program.account.hero.fetch(heroPDA);

    expect(heroDataAfter.trainingSlot.toNumber()).toBe(0);
    expect(heroDataAfter.xp.toNumber()).toBeGreaterThan(
      heroDataBefore.xp.toNumber()
    );
    expect(heroDataAfter.level.toNumber()).toBeGreaterThan(
      heroDataBefore.level.toNumber()
    );
  });

  it("Hero earned gold", async () => {
    const [heroPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from(HERO_SEED), player.publicKey.toBuffer()],
      anchor.workspace.IdleBattle.programId
    );

    const heroDataBefore = await program.account.hero.fetch(heroPDA);

    for (let i = 0; i < 3; i++) {
      const txTrain = await program.methods
        .train()
        .accounts({
          player: player.publicKey,
          hero: heroPDA,
        })
        .rpc();
      await confirmAndLogTransaction(provider.connection, txTrain);

      const txRecall = await program.methods
        .recall()
        .accounts({
          player: player.publicKey,
          hero: heroPDA,
        })
        .rpc();
      await confirmAndLogTransaction(provider.connection, txRecall);
    }

    const heroDataAfter = await program.account.hero.fetch(heroPDA);

    expect(heroDataAfter.trainingSlot.toNumber()).toBe(0);
    expect(heroDataAfter.gold.toNumber()).toBeGreaterThan(
      heroDataBefore.gold.toNumber()
    );
    expect(heroDataAfter.level.toNumber()).toBeGreaterThan(
      heroDataBefore.level.toNumber()
    );
  });

  it("Hero stats reset", async () => {
    const [heroPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from(HERO_SEED), player.publicKey.toBuffer()],
      anchor.workspace.IdleBattle.programId
    );
    const tx = await program.methods
      .resetstats()
      .accounts({
        player: player.publicKey,
        hero: heroPDA,
      })
      .rpc();
    await confirmAndLogTransaction(provider.connection, tx);

    const heroDataAfter = await program.account.hero.fetch(heroPDA);

    expect(heroDataAfter.trainingSlot.toNumber()).toBe(0);
    expect(heroDataAfter.xp.toNumber()).toBe(0);
    expect(heroDataAfter.level.toNumber()).toBe(0);
    expect(heroDataAfter.gold.toNumber()).toBe(0);
  });
});
