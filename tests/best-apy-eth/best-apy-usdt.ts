import * as anchor from "@project-serum/anchor";
import * as spl from "@solana/spl-token";
import { assert } from "chai";
import {
  GoblinGold,
  NetworkName,
  Protocols,
  TokenName,
  decodeAccount,
} from "goblin-sdk-local";
import { BestApy } from "../target/types/best_apy";

const ETH_MINT_PUBKEY = new anchor.web3.PublicKey(
  "2FPyTwcZLUg1MDrwsyoP4D6s1tM7hAkHYRjkNb5w6Pxk"
);

const TOKEN_INPUT = TokenName.ETH;
const PROTOCOLS = [
  Protocols.Mango,
  Protocols.Solend,
  // Protocols.Port,
  Protocols.Tulip,
  Protocols.Francium,
];

const WEIGHTS_SCALE = 10_000;

describe("best_apy (ETH)", () => {
  const testingProgram = anchor.workspace.BestApy as anchor.Program<BestApy>;

  const client = new GoblinGold(
    NetworkName.Mainnet,
    undefined,
    anchor.Provider.local(),
    testingProgram.programId
  );

  const program = client.BestApy;
  const userSigner = program.provider.wallet.publicKey;

  client.setToken(TOKEN_INPUT);

  it("Initialize vault with weights", async () => {
    const protocolWeights = PROTOCOLS.map((_, indx) => {
      const len = PROTOCOLS.length;
      const w = Math.floor(WEIGHTS_SCALE / len);
      return indx === 0 ? WEIGHTS_SCALE - w * (len - 1) : w;
    });

    const tx = await program.initializeVault(new anchor.BN(0));
    for (const protocol of PROTOCOLS) {
      tx.add(
        await program.methods
          .addProtocol(protocol)
          .accounts({
            userSigner,
            vaultAccount: program.vaultKeys[TOKEN_INPUT].vaultAccount,
          })
          .transaction()
      );
    }
    tx.add(await program.setProtocolWeights(protocolWeights));

    const txSig = await program.provider.send(tx);

    const vaultData = await program.decodeVault();
    const vaultWeights = vaultData.protocols.map((data) => data.weight);
    const vaultProtocols = vaultData.protocols.map((data) => data.protocolId);

    assert.deepStrictEqual(vaultWeights, protocolWeights);
    assert.deepStrictEqual(vaultProtocols, PROTOCOLS);
  });

  it("Initialize protocol accounts", async () => {
    const txsProtocols = await program.initializeProtocolAccounts();
    for (const tx of txsProtocols) {
      const txSig = await program.provider.send(tx);
    }
  });

  it("Set hashes", async () => {
    const txsHashes = await program.setHashes();
    const txHashes = txsHashes.reduce(
      (acc, tx) => acc.add(tx),
      new anchor.web3.Transaction()
    );

    const txSigHashes = await program.provider.send(txHashes);
  });

  it("Deposit", async () => {
    const amount = new anchor.BN(1_000_000_000);

    const userInputTokenAccount = await spl.getAssociatedTokenAddress(
      ETH_MINT_PUBKEY,
      userSigner,
      false
    );

    const userLpTokenAccount = await spl.getAssociatedTokenAddress(
      program.vaultKeys[TOKEN_INPUT].vaultLpTokenMintAddress,
      userSigner,
      false
    );

    const tx = new anchor.web3.Transaction()
      .add(
        spl.createAssociatedTokenAccountInstruction(
          userSigner,
          userLpTokenAccount,
          userSigner,
          program.vaultKeys[TOKEN_INPUT].vaultLpTokenMintAddress
        )
      )
      .add(
        await program.deposit({
          userInputTokenAccount,
          userLpTokenAccount,
          amount,
        })
      );

    const txsAll = await program.provider.send(tx);
  });

  it("Disable Mango", async () => {
    const protocolWeights = [0, 2500, 2500, 2500, 2500];
    const tx = await program.setProtocolWeights(protocolWeights);
    const txSig = await program.provider.send(tx);
  });

  it("Deposit into the protocols", async () => {
    const txs = await program.rebalance();
    for (const tx of txs) {
      const txSig = await program.provider.send(tx);
    }
  });

  it("Refresh weights", async () => {
    const tx = await program.refreshWeights();
    const txSig = await program.provider.send(tx);
  });

  it("Withdraw from the protocols", async () => {
    const userInputTokenAccount = await spl.getAssociatedTokenAddress(
      ETH_MINT_PUBKEY,
      userSigner,
      false
    );

    const userLpTokenAccount = await spl.getAssociatedTokenAddress(
      program.vaultKeys[TOKEN_INPUT].vaultLpTokenMintAddress,
      userSigner,
      false
    );

    const userLpTokenAccountInfo =
      await program.provider.connection.getAccountInfo(userLpTokenAccount);
    if (!userLpTokenAccountInfo) {
      throw new Error("Error: user_lp_token_account not found");
    }

    const data = decodeAccount(userLpTokenAccountInfo.data);
    const lpAmount = new anchor.BN(data.amount);

    const txs = await program.withdraw({
      userInputTokenAccount,
      userLpTokenAccount,
      lpAmount,
    });

    for (const tx of txs) {
      const txSig = await program.provider.send(tx);
    }
  });
});
