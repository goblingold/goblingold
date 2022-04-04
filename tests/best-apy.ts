import * as anchor from "@project-serum/anchor";
import * as spl from "@solana/spl-token";
import { assert } from "chai";
import { GoblinGold, NetworkName, Protocols, TokenName } from "goblin-sdk";
import { BestApy } from "../target/types/best_apy";

describe("best_apy", () => {
  const testingProgram = anchor.workspace.BestApy as anchor.Program<BestApy>;

  const client = new GoblinGold(
    NetworkName.Mainnet,
    undefined,
    anchor.Provider.local(),
    testingProgram.programId
  );

  const program = client.BestApy;
  const tokenInput = TokenName.WSOL;

  it("Initialize vault with weights", async () => {
    const protocolWeights = [2000, 2000, 2000, 2000, 2000];

    const txVault = await program.initializeVault();
    const txWeights = await program.setProtocolWeights(protocolWeights);
    txVault.add(txWeights);

    const txSigVault = await program.provider.send(txVault);
    console.log("tx init_vault:", txSigVault);

    const vaultData = await program.decodeVault();
    const vaultWeights = vaultData.protocols.map((data) => data.weight);

    assert.deepStrictEqual(vaultWeights, protocolWeights);
  });

  it("Initialize protocol accounts", async () => {
    const txsProtocols = await program.initializeProtocolAccounts();
    for (let i = 0; i < txsProtocols.length; ++i) {
      const txSig = await program.provider.send(txsProtocols[i]);
      console.log("tx init_protocols_" + i.toString() + ":", txSig);
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
    const amount = new anchor.BN(1_000_000_000);
    const userSigner = program.provider.wallet.publicKey;

    const wrappedKeypair = anchor.web3.Keypair.generate();
    const userInputTokenAccount = wrappedKeypair.publicKey;

    const userLpTokenAccount = await spl.getAssociatedTokenAddress(
      program.vaultKeys[tokenInput].vaultLpTokenMintAddress,
      userSigner,
      false
    );

    const lamports = await spl.getMinimumBalanceForRentExemptAccount(
      client.provider.connection
    );

    const tx = new anchor.web3.Transaction()
      .add(
        anchor.web3.SystemProgram.createAccount({
          fromPubkey: userSigner,
          newAccountPubkey: userInputTokenAccount,
          space: spl.ACCOUNT_SIZE,
          lamports,
          programId: spl.TOKEN_PROGRAM_ID,
        }),
        anchor.web3.SystemProgram.transfer({
          fromPubkey: userSigner,
          toPubkey: userInputTokenAccount,
          lamports: amount.toNumber(),
        }),
        spl.createInitializeAccountInstruction(
          userInputTokenAccount,
          spl.NATIVE_MINT,
          userSigner
        )
      )
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
      )
      .add(
        spl.createCloseAccountInstruction(
          userInputTokenAccount,
          userSigner,
          userSigner,
          []
        )
      );

    const txsAll = await program.provider.send(tx, [wrappedKeypair]);
    console.log("tx deposit:", txsAll);
  });

  it("Deposit into the protocols", async () => {
    const txs = await program.rebalance();
    for (let i = 0; i < txs.length; ++i) {
      if (i != Protocols.Mango) {
        const txSig = await program.provider.send(txs[i]);
        console.log("tx deposit_protocols_" + i.toString() + ":", txSig);
      }
    }
  });
});
