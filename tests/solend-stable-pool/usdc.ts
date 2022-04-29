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

const USDC_MINT_PUBKEY = new anchor.web3.PublicKey(
  "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
);

describe("best_apy: solend-isolated-pool (USDC)", () => {
  const testingProgram = anchor.workspace.BestApy as anchor.Program<BestApy>;

  const client = new GoblinGold(
    NetworkName.Mainnet,
    undefined,
    anchor.Provider.local(),
    testingProgram.programId
  );
  client.setToken(TokenName.USDC);

  const program = client.BestApy;
  const tokenInput = TokenName.USDC;
  const userSigner = program.provider.wallet.publicKey;

  it("Initialize solend isolated pool", async () => {
    const tx = new anchor.web3.Transaction()
      .add(await program.initializeVault())
      .add(
        await program.methods
          .addProtocol(Protocols.SolendStablePool)
          .accounts({
            userSigner,
            vaultAccount: program.vaultKeys[tokenInput].vaultAccount,
          })
          .transaction()
      )
      .add(await program.setProtocolWeights([10_000]));

    const txSig = await program.provider.send(tx);

    const vaultData = await program.decodeVault();
    const vaultWeights = vaultData.protocols.map((data) => data.weight);
    const vaultProtocols = vaultData.protocols.map((data) => data.protocolId);

    assert.deepStrictEqual(vaultWeights, [10_000]);
    assert.deepStrictEqual(vaultProtocols, [Protocols.SolendStablePool]);
  });

  it("Initialize protocol accounts", async () => {
    const txsProtocols = await program.initializeProtocolAccounts();
    for (let i = 0; i < txsProtocols.length; ++i) {
      const txSig = await program.provider.send(txsProtocols[i]);
    }
  });

  it("Set hashes", async () => {
    const txsHashes = await program.setHashes();
    const tx = txsHashes.reduce(
      (acc, tx) => acc.add(tx),
      new anchor.web3.Transaction()
    );

    const txSig = await program.provider.send(tx);
  });

  it("Deposit", async () => {
    const amount = new anchor.BN(1_000_000);

    const userInputTokenAccount = await spl.getAssociatedTokenAddress(
      USDC_MINT_PUBKEY,
      userSigner,
      false
    );

    const userLpTokenAccount = await spl.getAssociatedTokenAddress(
      program.vaultKeys[tokenInput].vaultLpTokenMintAddress,
      userSigner,
      false
    );

    const tx = new anchor.web3.Transaction()
      .add(
        spl.createAssociatedTokenAccountInstruction(
          userSigner,
          userLpTokenAccount,
          userSigner,
          program.vaultKeys[tokenInput].vaultLpTokenMintAddress
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

  it("Deposit into solend isolated pool", async () => {
    const txs = await program.rebalance();
    for (let i = 0; i < txs.length; ++i) {
      const txSig = await program.provider.send(txs[i]);
    }
  });

  it("Refresh weights", async () => {
    const tx = await program.refreshWeights();
    const txSig = await program.provider.send(tx);
  });

  it("Withdraw from the isolated pool", async () => {
    const userInputTokenAccount = await spl.getAssociatedTokenAddress(
      USDC_MINT_PUBKEY,
      userSigner,
      false
    );

    const userLpTokenAccount = await spl.getAssociatedTokenAddress(
      program.vaultKeys[tokenInput].vaultLpTokenMintAddress,
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

    for (let i = 0; i < txs.length; ++i) {
      const tx = new anchor.web3.Transaction().add(txs[i]);
      const txSig = await program.provider.send(tx);
    }
  });
});
