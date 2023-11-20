import { error, info, log } from "console";
import fs from "fs";

import { AnchorProvider, Program, Wallet } from "@coral-xyz/anchor";
import {
  Connection,
  Keypair,
  PublicKey,
  TransactionSignature,
} from "@solana/web3.js";

// Read content of key file
const KEY_FILE = "/Users/bruno/.config/solana/id.json";
// const KEY_FILE = "/Users/bruno/Personas/chainsona/chainsona-dev.key";
const keypair = Keypair.fromSecretKey(
  Buffer.from(
    JSON.parse(fs.readFileSync("/Users/bruno/.config/solana/id.json", "utf-8"))
  )
);
log(`Wallet: ${keypair.publicKey.toBase58()}` + "\n");

// Create a wallet object from a keypair
const wallet = new Wallet(keypair);

// Connect to Solana (localnet)
const connection = new Connection(
  "https://devnet.helius-rpc.com/?api-key=979f572d-5e7b-46ca-8c97-81c87caf9bba",
  "confirmed"
);

// Create an Anchor provider
const provider = new AnchorProvider(connection, wallet, {});

// PROGRAM INFORMATION
const IDLE_BATTLE_BATTLE_SEED = "battle";
const IDLE_BATTLE_HERO_SEED = "hero";
const IDLE_BATTLE_PROGRAM_ID = "EZhEi1iBCbUyT3tdiMGpye1GVqSzCcSPWeyYxEZ6drn2";

const programId = new PublicKey(IDLE_BATTLE_PROGRAM_ID);

// Import the generated IDL
import { IDL, IdleBattle } from "./idl/idle_battle";

// Create an Anchor Program from the IDL.
const program = new Program(IDL as IdleBattle, programId, provider);

const [heroPDA] = PublicKey.findProgramAddressSync(
  [Buffer.from(IDLE_BATTLE_HERO_SEED), wallet.publicKey.toBuffer()],
  programId
);

const [battlePDA] = PublicKey.findProgramAddressSync(
  [Buffer.from(IDLE_BATTLE_BATTLE_SEED)],
  programId
);

async function confirmAndLogTransaction(txHash: TransactionSignature) {
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

  info(
    `Solana Explorer: https://explorer.solana.com/tx/${txHash}?cluster=devnet`
  );
}

async function train() {
  log(`GO TRAIN! (PDA: ${heroPDA.toBase58()})\n`);
  let heroData: any;

  try {
    heroData = await program.account.hero.fetch(heroPDA);
  } catch (e) {
    log("Initialize Hero");
    const tx = await program.methods
      .initialize()
      .accounts({
        player: wallet.publicKey,
        hero: heroPDA,
      })
      .rpc();
    await confirmAndLogTransaction(tx);
    heroData = await program.account.hero.fetch(heroPDA);
  }

  if (heroData.trainingSlot > 0) {
    return error("Already training");
  }

  const tx = await program.methods
    .train()
    .accounts({
      hero: heroPDA,
      player: wallet.publicKey,
    })
    .rpc();
  await confirmAndLogTransaction(tx);

  heroData = await program.account.hero.fetch(heroPDA);
  log(`Training Slot: ${heroData.trainingSlot}`);
}

async function resetStats() {
  log(`RESET STATS! (PDA: ${heroPDA.toBase58()})\n`);
  let heroData = await program.account.hero.fetch(heroPDA);

  const tx = await program.methods
    .resetstats()
    .accounts({
      hero: heroPDA,
      player: wallet.publicKey,
    })
    .rpc();
  await confirmAndLogTransaction(tx);

  const newHeroData = await program.account.hero.fetch(heroPDA);
  log("newHeroData", newHeroData);
}

async function recall() {
  log(`RECALL! (PDA: ${heroPDA.toBase58()})\n`);
  let heroData = await program.account.hero.fetch(heroPDA);

  if (heroData.trainingSlot === 0) {
    return error("Hero is not training");
  }

  const tx = await program.methods
    .recall()
    .accounts({
      hero: heroPDA,
      player: wallet.publicKey,
    })
    .rpc();
  await confirmAndLogTransaction(tx);

  const newHeroData = await program.account.hero.fetch(heroPDA);
  log(`Hero gains ${newHeroData.xp - heroData.xp} (XP: ${newHeroData.xp})`);
  if (newHeroData.level > heroData.level) {
    log(`Hero levels up to ${newHeroData.level}`);
  }
  log(`Hero earns ${heroData.gold} $GOLD`);
}

async function battle() {
  log(`FIIIGHT! (PDA: ${battlePDA.toBase58()})\n`);
  let battleData = await program.account.battleRound.fetch(battlePDA);

  if (battleData.enemy.toNumber() === 0 || battleData.hero.toNumber() === 0) {
    const tx = await program.methods
      .battle()
      .accounts({
        battle: battlePDA,
        hero: heroPDA,
        player: wallet.publicKey,
      })
      .rpc();
    await confirmAndLogTransaction(tx);

    battleData = await program.account.battleRound.fetch(battlePDA);
    log(
      `Health (Enemy VS Hero): ${battleData.enemy.toNumber()} VS ${battleData.hero.toNumber()}`
    );
  }

  await attackLoop();
}

async function attackLoop() {
  let battleData = await program.account.battleRound.fetch(battlePDA);

  try {
    while (battleData.enemy.toNumber() > 0 && battleData.hero.toNumber() > 0) {
      // log("Attacking");
      const tx = await program.methods
        .attack()
        .accounts({
          battleRound: battlePDA,
          player: wallet.publicKey,
        })
        .rpc();
      await confirmAndLogTransaction(tx);

      battleData = await program.account.battleRound.fetch(battlePDA);
      switch (true) {
        case battleData.enemy.toNumber() === 0:
          log("HERO WINS!" + "\n");
          break;

        case battleData.hero.toNumber() === 0:
          log("HERO DIED!" + "\n");
          break;

        default:
          log(
            `Health (Enemy VS Hero): ${battleData.enemy.toNumber()} VS ${battleData.hero.toNumber()}`
          );
          // log("Enemy Health: ", battleData.enemy.toNumber());
          // log("Hero Health: ", battleData.hero.toNumber());
          break;
      }
    }
  } catch (e) {
    log(e);
  }
}

(async () => {
  await resetStats();
  // await train();
  // await recall();
  // await battle();
})();
