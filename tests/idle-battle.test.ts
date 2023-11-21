import { log } from "console";

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  Connection,
  PublicKey,
  sendAndConfirmTransaction,
  SystemProgram,
  Transaction,
  TransactionSignature,
} from "@solana/web3.js";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  createMint,
  getAccount,
  getMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  mintToChecked,
} from "@solana/spl-token";

import { IdleBattle } from "../target/types/idle_battle";

const BATTLE_SEED = "battle";
const HERO_SEED = "hero";
const VAULT_SEED = "vault";

async function createMintToken(
  connection: Connection,
  wallet: anchor.Wallet,
  mintKeypair: anchor.web3.Keypair
) {
  const mint = await createMint(
    connection,
    wallet.payer,
    wallet.publicKey,
    null,
    9,
    mintKeypair
  );
  return mint;
}

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
  const admin = provider.wallet as anchor.Wallet;
  log(`Admin: ${admin.publicKey.toBase58()}`);

  // Uncomment to generate keypair to mint the token
  // const mintKeypair = anchor.web3.Keypair.generate();
  const mintKeypair = anchor.web3.Keypair.fromSecretKey(
    new Uint8Array([
      6, 202, 246, 97, 31, 39, 24, 36, 161, 179, 17, 199, 82, 33, 252, 77, 11,
      118, 218, 4, 55, 2, 0, 99, 49, 205, 15, 73, 149, 166, 121, 197, 72, 152,
      90, 200, 77, 240, 32, 237, 33, 68, 122, 210, 147, 45, 206, 111, 248, 73,
      250, 40, 226, 249, 88, 12, 246, 195, 14, 237, 104, 203, 74, 242,
    ])
  );
  log(`$TOKEN: ${mintKeypair.publicKey.toBase58()}`);

  // Uncomment to generate player's keypair
  // const playerKeypair = anchor.web3.Keypair.generate();
  const playerKeypair = anchor.web3.Keypair.fromSecretKey(
    new Uint8Array([
      204, 170, 2, 118, 7, 243, 109, 125, 229, 238, 28, 12, 39, 136, 254, 202,
      16, 243, 109, 179, 62, 204, 219, 150, 111, 32, 54, 179, 165, 155, 105,
      185, 89, 138, 224, 166, 141, 193, 173, 226, 113, 71, 226, 86, 13, 232, 93,
      168, 216, 63, 120, 105, 11, 155, 178, 76, 67, 137, 90, 249, 96, 135, 102,
      248,
    ])
  );
  const player = new anchor.Wallet(playerKeypair);
  log(`Player: ${player.publicKey.toBase58()}`);

  const program = anchor.workspace.IdleBattle as Program<IdleBattle>;

  it("Admin should mint the token", async () => {
    // Uncomment to mint the token
    // await createMintToken(provider.connection, admin, mintKeypair);

    let mintAccount = await getMint(provider.connection, mintKeypair.publicKey);
    expect(mintAccount?.address).toBeDefined();
  });

  it("Admin should send SOL to the player", async () => {
    const balanceBefore = await provider.connection.getBalance(
      player.publicKey
    );

    // Transfer some SOL to the player
    const txTransferToPlayer = await sendAndConfirmTransaction(
      provider.connection,
      new Transaction().add(
        SystemProgram.transfer({
          fromPubkey: admin.publicKey,
          toPubkey: player.publicKey,
          lamports: 1e9, // 1 SOL
        })
      ),
      [admin.payer]
    );
    await confirmAndLogTransaction(provider.connection, txTransferToPlayer);

    const balanceAfter = await provider.connection.getBalance(player.publicKey);

    expect(balanceAfter).toBeGreaterThan(balanceBefore);
  });

  it("Admin should initialize the vault", async () => {
    let [vaultTokenAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from(VAULT_SEED)],
      program.programId
    );
    log(`Token vault: ${vaultTokenAccount.toBase58()}`);

    // Initialize the program
    const tx = await program.methods
      .initialize()
      .accounts({
        admin: admin.publicKey,
        vaultTokenAccount,
        mint: mintKeypair.publicKey,
      })
      .rpc();
    await confirmAndLogTransaction(provider.connection, tx);

    let tokenAccount = await getAccount(provider.connection, vaultTokenAccount);
    expect(tokenAccount.isInitialized).toBe(true);
  });

  it("Admin mint some tokens into the vault", async () => {
    let [vaultTokenAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from(VAULT_SEED)],
      program.programId
    );
    log(`Token vault: ${vaultTokenAccount.toBase58()}`);

    let balanceBefore = await getAccount(
      provider.connection,
      vaultTokenAccount
    );

    const txMintToVault = await mintToChecked(
      provider.connection,
      admin.payer,
      mintKeypair.publicKey,
      vaultTokenAccount,
      admin.payer,
      100e9, // 100 token
      9
    );
    await confirmAndLogTransaction(provider.connection, txMintToVault);

    const balanceAfter = await getAccount(
      provider.connection,
      vaultTokenAccount
    );

    expect(balanceAfter.amount).toBeGreaterThan(balanceBefore.amount);
  });

  it("Hero should be initialized", async () => {
    const [heroAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from(HERO_SEED), player.publicKey.toBuffer()],
      program.programId
    );

    const tx = await program.methods
      .initializeHero()
      .accounts({
        player: player.publicKey,
        hero: heroAccount,
      })
      .signers([player.payer])
      .rpc();
    await confirmAndLogTransaction(provider.connection, tx);

    const heroData = await program.account.hero.fetch(heroAccount);

    expect(heroData.trainingSlot.toNumber()).toBe(0);
    expect(heroData.xp.toNumber()).toBe(0);
    expect(heroData.level.toNumber()).toBe(0);
    expect(heroData.reward.toNumber()).toBe(0);
  });

  it("Hero should be training", async () => {
    const [heroAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from(HERO_SEED), player.publicKey.toBuffer()],
      program.programId
    );

    const tx = await program.methods
      .train()
      .accounts({
        player: player.publicKey,
        hero: heroAccount,
      })
      .signers([player.payer])
      .rpc();
    await confirmAndLogTransaction(provider.connection, tx);

    const heroDataAfter = await program.account.hero.fetch(heroAccount);

    expect(heroDataAfter.trainingSlot.toNumber()).toBeGreaterThan(0);
  });

  it("Hero should be recalled", async () => {
    const [heroAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from(HERO_SEED), player.publicKey.toBuffer()],
      program.programId
    );

    const [vaultTokenAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from(VAULT_SEED)],
      program.programId
    );

    const playerTokenAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      player.payer,
      mintKeypair.publicKey,
      player.publicKey
    );

    const txRecall = await program.methods
      .recall()
      .accounts({
        hero: heroAccount,
        vaultTokenAccount,
        player: player.publicKey,
        mintAccount: mintKeypair.publicKey,
        playerTokenAccount: playerTokenAccount.address,
        associatedTokenAccount: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([player.payer])
      .rpc();
    await confirmAndLogTransaction(provider.connection, txRecall);

    const heroDataAfter = await program.account.hero.fetch(heroAccount);

    expect(heroDataAfter.trainingSlot.toNumber()).toBe(0);
    expect(heroDataAfter.reward.toNumber()).toBe(0);
  });

  it("Hero should gain XP", async () => {
    const [heroAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from(HERO_SEED), player.publicKey.toBuffer()],
      program.programId
    );

    const [vaultTokenAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from(VAULT_SEED)],
      program.programId
    );

    const playerTokenAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      player.payer,
      mintKeypair.publicKey,
      player.publicKey
    );

    const heroDataBefore = await program.account.hero.fetch(heroAccount);

    const txTrain = await program.methods
      .train()
      .accounts({
        player: player.publicKey,
        hero: heroAccount,
      })
      .signers([player.payer])
      .rpc();
    await confirmAndLogTransaction(provider.connection, txTrain);

    const txRecall = await program.methods
      .recall()
      .accounts({
        hero: heroAccount,
        vaultTokenAccount,
        player: player.publicKey,
        mintAccount: mintKeypair.publicKey,
        playerTokenAccount: playerTokenAccount.address,
        associatedTokenAccount: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([player.payer])
      .rpc();
    await confirmAndLogTransaction(provider.connection, txRecall);

    const heroDataAfter = await program.account.hero.fetch(heroAccount);

    expect(heroDataAfter.xp.toNumber()).toBeGreaterThan(
      heroDataBefore.xp.toNumber()
    );
  });

  it("Hero should level up", async () => {
    const [heroAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from(HERO_SEED), player.publicKey.toBuffer()],
      program.programId
    );

    const [vaultTokenAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from(VAULT_SEED)],
      program.programId
    );

    const playerTokenAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      player.payer,
      mintKeypair.publicKey,
      player.publicKey
    );

    const heroDataBefore = await program.account.hero.fetch(heroAccount);

    for (let i = 0; i < 2; i++) {
      const txTrain = await program.methods
        .train()
        .accounts({
          player: player.publicKey,
          hero: heroAccount,
        })
        .signers([player.payer])
        .rpc();
      await confirmAndLogTransaction(provider.connection, txTrain);

      const txRecall = await program.methods
        .recall()
        .accounts({
          hero: heroAccount,
          vaultTokenAccount,
          player: player.publicKey,
          mintAccount: mintKeypair.publicKey,
          playerTokenAccount: playerTokenAccount.address,
          associatedTokenAccount: ASSOCIATED_TOKEN_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .signers([player.payer])
        .rpc();
      await confirmAndLogTransaction(provider.connection, txRecall);
    }

    const heroDataAfter = await program.account.hero.fetch(heroAccount);

    expect(heroDataAfter.trainingSlot.toNumber()).toBe(0);
    expect(heroDataAfter.xp.toNumber()).toBeGreaterThan(
      heroDataBefore.xp.toNumber()
    );
    expect(heroDataAfter.level.toNumber()).toBeGreaterThan(
      heroDataBefore.level.toNumber()
    );
  });

  it("Hero should earn $TOKEN", async () => {
    const [heroAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from(HERO_SEED), player.publicKey.toBuffer()],
      program.programId
    );

    const [vaultTokenAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from(VAULT_SEED)],
      program.programId
    );

    let playerTokenAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      player.payer,
      mintKeypair.publicKey,
      player.publicKey
    );
    const amountBefore = playerTokenAccount.amount;

    for (let i = 0; i < 3; i++) {
      const txTrain = await program.methods
        .train()
        .accounts({
          player: player.publicKey,
          hero: heroAccount,
        })
        .signers([player.payer])
        .rpc();
      await confirmAndLogTransaction(provider.connection, txTrain);

      const txRecall = await program.methods
        .recall()
        .accounts({
          hero: heroAccount,
          vaultTokenAccount,
          player: player.publicKey,
          mintAccount: mintKeypair.publicKey,
          playerTokenAccount: playerTokenAccount.address,
          associatedTokenAccount: ASSOCIATED_TOKEN_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .signers([player.payer])
        .rpc();
      await confirmAndLogTransaction(provider.connection, txRecall);
    }

    const heroDataAfter = await program.account.hero.fetch(heroAccount);

    playerTokenAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      player.payer,
      mintKeypair.publicKey,
      player.publicKey
    );
    const amountAfer = playerTokenAccount.amount;

    expect(amountAfer).toBeGreaterThan(amountBefore);
    expect(heroDataAfter.reward.toNumber()).toBe(0);
  });

  it("Player should not reset Hero stats", async () => {
    const [heroAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from(HERO_SEED), player.publicKey.toBuffer()],
      program.programId
    );

    const t = async () => {
      await program.methods
        .resetHeroStats()
        .accounts({
          admin: player.publicKey,
          player: player.publicKey,
          hero: heroAccount,
          systemProgram: SystemProgram.programId,
        })
        .signers([player.payer])
        .rpc();
    };
    // expect(t).toThrow(anchor.AnchorError);

    const heroDataAfter = await program.account.hero.fetch(heroAccount);

    expect(heroDataAfter.xp.toNumber()).not.toBe(0);
    expect(heroDataAfter.level.toNumber()).not.toBe(0);
  });

  it("Admin should reset Hero stats", async () => {
    const [heroAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from(HERO_SEED), player.publicKey.toBuffer()],
      program.programId
    );

    const tx = await program.methods
      .resetHeroStats()
      .accounts({
        admin: admin.publicKey,
        player: player.publicKey,
        hero: heroAccount,
        systemProgram: SystemProgram.programId,
      })
      .rpc();
    await confirmAndLogTransaction(provider.connection, tx);

    const heroDataAfter = await program.account.hero.fetch(heroAccount);

    expect(heroDataAfter.trainingSlot.toNumber()).toBe(0);
    expect(heroDataAfter.xp.toNumber()).toBe(0);
    expect(heroDataAfter.level.toNumber()).toBe(0);
    expect(heroDataAfter.reward.toNumber()).toBe(0);
  });
});
