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
  const userSigner = program.provider.wallet.publicKey;
  console.log(program.vaultKeys[tokenInput].vaultAccount.toString());

  it("Initialize vault with weights", async () => {
    const protocolWeights = [2000, 2000, 2000, 2000, 2000];
    const protocolList = [
      Protocols.Mango,
      Protocols.Solend,
      Protocols.Port,
      Protocols.Tulip,
      Protocols.Francium,
    ];

    const tx = new anchor.web3.Transaction().add(
      await program.initializeVault(new anchor.BN(0))
    );

    for (const protocol of protocolList) {
      tx.add(
        await program.methods
          .addProtocol(protocol)
          .accounts({
            userSigner,
            vaultAccount: program.vaultKeys[tokenInput].vaultAccount,
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
    assert.deepStrictEqual(vaultProtocols, protocolList);
  });

  console.log(program.vaultKeys[tokenInput].vaultAccount.toString());

  it("Initialize protocol accounts", async () => {
    const txsProtocols = await program.initializeProtocolAccounts();
    for (let i = 0; i < txsProtocols.length; ++i) {
      const txSig = await program.provider.send(txsProtocols[i]);
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
  });

  it("Disable Mango & Port", async () => {
    const protocolWeights = [0, 6000, 0, 2000, 2000];
    const tx = await program.setProtocolWeights(protocolWeights);
    const txSig = await program.provider.send(tx);
  });

  it("Deposit into the protocols", async () => {
    const txs = await program.rebalance();
    for (let i = 0; i < txs.length; ++i) {
      const txSig = await program.provider.send(txs[i]);
    }
  });

  it("Refresh weights", async () => {
    const tx = await program.refreshWeights();
    const txSig = await program.provider.send(tx);
  });

  it("Withdraw from the protocols", async () => {
    const userSigner = program.provider.wallet.publicKey;

    const wrappedKeypair = anchor.web3.Keypair.generate();
    const userInputTokenAccount = wrappedKeypair.publicKey;

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

    const lamports = await spl.getMinimumBalanceForRentExemptAccount(
      client.provider.connection
    );

    for (let i = 0; i < txs.length; ++i) {
      const tx = new anchor.web3.Transaction()
        .add(
          anchor.web3.SystemProgram.createAccount({
            fromPubkey: userSigner,
            newAccountPubkey: userInputTokenAccount,
            space: spl.ACCOUNT_SIZE,
            lamports,
            programId: spl.TOKEN_PROGRAM_ID,
          }),
          spl.createInitializeAccountInstruction(
            userInputTokenAccount,
            spl.NATIVE_MINT,
            userSigner
          )
        )
        .add(txs[i])
        .add(
          spl.createCloseAccountInstruction(
            userInputTokenAccount,
            userSigner,
            userSigner,
            []
          )
        );

      const txSig = await program.provider.send(tx, [wrappedKeypair]);
    }
  });
});
