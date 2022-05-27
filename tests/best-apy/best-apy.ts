import * as anchor from "@project-serum/anchor";
import * as spl from "@solana/spl-token";
import { assert } from "chai";
import { GoblinGold, Protocols, TOKENS, decodeAccount } from "goblin-sdk-local";
import { getProtocols } from "./protocols";

const INPUT_TOKEN = process.env.ASSET;
const INPUT_TOKEN_MINT = new anchor.web3.PublicKey(
  TOKENS[INPUT_TOKEN].mintAddress
);

const PROTOCOLS = getProtocols(INPUT_TOKEN);

const WEIGHTS_SCALE = 10_000;
const WEIGHTS = PROTOCOLS.map((_, indx) => {
  const len = PROTOCOLS.length;
  const weight = Math.floor(WEIGHTS_SCALE / len);
  return indx === 0 ? WEIGHTS_SCALE - weight * (len - 1) : weight;
});

const CONFIRM_OPTS: anchor.web3.ConfirmOptions = {
  skipPreflight: true,
};

describe("best_apy (" + INPUT_TOKEN + ")", () => {
  const provider = anchor.Provider.local();
  const userSigner = provider.wallet.publicKey;

  const client = new GoblinGold({
    connection: provider.connection,
    wallet: provider.wallet,
  });

  const program = client.BestApy;

  program.setToken(INPUT_TOKEN);

  it("Initialize vault with weights", async () => {
    const tx = await program.initializeVault(new anchor.BN(0));
    for (const protocol of PROTOCOLS) {
      tx.add(
        await program.methods
          .addProtocol(protocol)
          .accounts({
            userSigner,
            vaultAccount: program.vaultKeys[INPUT_TOKEN].vaultAccount,
          })
          .transaction()
      );
    }
    tx.add(await program.setProtocolWeights(WEIGHTS));

    await program.provider.sendAndConfirm(tx, [], CONFIRM_OPTS);
    const vaultData = await program.decodeVault();
    const vaultWeights = vaultData.protocols.map((data) => data.weight);
    const vaultProtocols = vaultData.protocols.map((data) => data.protocolId);

    assert.deepStrictEqual(vaultWeights, WEIGHTS);
    assert.deepStrictEqual(vaultProtocols, PROTOCOLS);
  });

  it("Initialize protocol accounts", async () => {
    const txs = await program.initializeProtocolAccounts();
    await Promise.all(
      txs.map(async (tx) =>
        program.provider.sendAndConfirm(tx, [], CONFIRM_OPTS)
      )
    );
  });

  it("Set hashes", async () => {
    const txsHashes = await program.setHashes();
    const txHashes = txsHashes.reduce(
      (acc, tx) => acc.add(tx),
      new anchor.web3.Transaction()
    );
    await program.provider.sendAndConfirm(txHashes, [], CONFIRM_OPTS);
  });

  it("Deposit", async () => {
    const amount = new anchor.BN(1_000_000_000);

    const userLpTokenAccount = await spl.getAssociatedTokenAddress(
      program.vaultKeys[INPUT_TOKEN].vaultLpTokenMintAddress,
      userSigner,
      false
    );

    const tx = new anchor.web3.Transaction();

    if (INPUT_TOKEN === "WSOL") {
      const wrappedKeypair = anchor.web3.Keypair.generate();
      const userInputTokenAccount = wrappedKeypair.publicKey;
      const lamports = await spl.getMinimumBalanceForRentExemptAccount(
        program.provider.connection
      );

      tx.add(
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
            program.vaultKeys[INPUT_TOKEN].vaultLpTokenMintAddress
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
      await program.provider.sendAndConfirm(tx, [wrappedKeypair], CONFIRM_OPTS);
    } else {
      const userInputTokenAccount = await spl.getAssociatedTokenAddress(
        INPUT_TOKEN_MINT,
        userSigner,
        false
      );

      tx.add(
        spl.createAssociatedTokenAccountInstruction(
          userSigner,
          userLpTokenAccount,
          userSigner,
          program.vaultKeys[INPUT_TOKEN].vaultLpTokenMintAddress
        )
      ).add(
        await program.deposit({
          userInputTokenAccount,
          userLpTokenAccount,
          amount,
        })
      );
      await program.provider.sendAndConfirm(tx, [], CONFIRM_OPTS);
    }
  });

  it("Disable Mango & Port", async () => {
    const iposMango = PROTOCOLS.findIndex((p) => p === Protocols.Mango);
    const iposPort = PROTOCOLS.findIndex((p) => p === Protocols.Port);

    for (const ipos of [iposMango, iposPort]) {
      if (ipos !== -1) {
        WEIGHTS[ipos] = 0;
      }
    }

    const weightsSum = WEIGHTS.reduce((acc, x) => acc + x, 0);
    const iposFirstNotNull = WEIGHTS.findIndex((w) => w != 0);

    WEIGHTS[iposFirstNotNull] += WEIGHTS_SCALE - weightsSum;

    const tx = await program.setProtocolWeights(WEIGHTS);
    await program.provider.sendAndConfirm(tx, [], CONFIRM_OPTS);
  });

  it("Deposit into the protocols", async () => {
    const [_txsWithdraw, txsDeposit] = await program.rebalance();
    await Promise.all(
      txsDeposit.map(async (tx) =>
        program.provider.sendAndConfirm(tx, [], CONFIRM_OPTS)
      )
    );
  });

  xit("Refresh weights", async () => {
    const tx = await program.refreshWeights();
    await program.provider.sendAndConfirm(tx, [], CONFIRM_OPTS);
  });

  it("Withdraw from the protocols", async () => {
    const userLpTokenAccount = await spl.getAssociatedTokenAddress(
      program.vaultKeys[INPUT_TOKEN].vaultLpTokenMintAddress,
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

    if (INPUT_TOKEN === "WSOL") {
      const wrappedKeypair = anchor.web3.Keypair.generate();
      const userInputTokenAccount = wrappedKeypair.publicKey;
      const lamports = await spl.getMinimumBalanceForRentExemptAccount(
        program.provider.connection
      );

      const txs = await program.withdraw({
        userInputTokenAccount,
        userLpTokenAccount,
        lpAmount,
      });

      for (const tx of txs) {
        const txAll = new anchor.web3.Transaction()
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
          .add(tx)
          .add(
            spl.createCloseAccountInstruction(
              userInputTokenAccount,
              userSigner,
              userSigner,
              []
            )
          );
        await program.provider.sendAndConfirm(
          txAll,
          [wrappedKeypair],
          CONFIRM_OPTS
        );
      }
    } else {
      const userInputTokenAccount = await spl.getAssociatedTokenAddress(
        INPUT_TOKEN_MINT,
        userSigner,
        false
      );

      const txs = await program.withdraw({
        userInputTokenAccount,
        userLpTokenAccount,
        lpAmount,
      });

      for (const tx of txs) {
        await program.provider.sendAndConfirm(tx, [], CONFIRM_OPTS);
      }
    }
  });
});
