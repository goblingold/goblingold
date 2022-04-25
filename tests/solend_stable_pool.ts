import * as anchor from "@project-serum/anchor";
import * as spl from "@solana/spl-token";
import { assert } from "chai";
import {
  GoblinGold,
  NetworkName,
  Protocols,
  TokenName,
  decodeAccount,
} from "wen-token-client";
import { BestApy } from "../target/types/best_apy";

describe("best_apy", () => {
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

  it("Initialize vault with weights", async () => {
    const txVault = await program.initializeVault();
    let protocolWeights = [2000, 2000, 2000, 2000, 1000, 1000];
    const txWeights = await program.setProtocolWeights(protocolWeights);
    txVault.add(txWeights);

    const txSigVault = await program.provider.send(txVault);
    console.log("tx init_vault:", txSigVault);

    const vaultData = await program.decodeVault();
    const vaultWeights = vaultData.protocols.map((data) => data.weight);

    assert.deepStrictEqual(vaultWeights, protocolWeights);
    protocolWeights = [0, 0, 0, 0, 0, 10000];
    const txSetProtocol = await program.provider.send(
      await program.setProtocolWeights(protocolWeights)
    );
    console.log("tx set protocol:", txSetProtocol);
  });

  it("Initialize protocol accounts", async () => {
    const txsProtocols = await program.initializeProtocolAccounts();
    for (let i = 0; i < txsProtocols.length; ++i) {
      const txSig = await program.provider.send(txsProtocols[i]);
      console.log("tx init_protocols_" + Protocols[i] + ":", txSig);
    }
  });

  it("Set hashes", async () => {
    const txsHashes = await program.setHashes();
    const txHashes = txsHashes.reduce(
      (acc, tx) => acc.add(tx),
      new anchor.web3.Transaction()
    );

    const txSigHashes = await program.provider.send(txHashes);
    console.log("tx set_hashes:", txSigHashes);
  });

  it("Deposit", async () => {
    const amount = new anchor.BN(1000);
    const userSigner = program.provider.wallet.publicKey;

    const userInputTokenAccount = await spl.getAssociatedTokenAddress(
      new anchor.web3.PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
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
    console.log("tx deposit:", txsAll);
  });

  it("Deposit into the protocols", async () => {
    const txs = await program.rebalance();
    for (let i = 0; i < txs.length; ++i) {
      const txSig = await program.provider.send(txs[i]);
      console.log("tx deposit_protocols_" + i.toString() + ":", txSig);
    }
  });

  it("Refresh weights", async () => {
    const tx = await program.refreshWeights();
    const txSig = await program.provider.send(tx);
    console.log("tx refresh:", txSig);
  });

  it("Withdraw from the protocols", async () => {
    const userSigner = program.provider.wallet.publicKey;

    const wrappedKeypair = anchor.web3.Keypair.generate();
    const userInputTokenAccount = await spl.getAssociatedTokenAddress(
      new anchor.web3.PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
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
      const tx = new anchor.web3.Transaction()
        .add(txs[i]);
      const txSig = await program.provider.send(tx);
      console.log("tx withdraw_protocols_" + i.toString() + ":", txSig);
    }
  });
});
