import * as anchor from "@project-serum/anchor";
import * as spl from "@solana/spl-token";
import { assert } from "chai";
import { GoblinGold, Protocols, TOKENS, decodeAccount } from "goblin-sdk-local";
import { getProtocols } from "./protocols";

const INPUT_TOKEN = "USDC";

const INPUT_TOKEN_MINT = new anchor.web3.PublicKey(
  TOKENS[INPUT_TOKEN].mintAddress
);

const BORROW_TOKEN = "WSOL";
const BORROW_TOKEN_MINT = new anchor.web3.PublicKey(
  TOKENS[BORROW_TOKEN].mintAddress
);
const CONFIRM_OPTS: anchor.web3.ConfirmOptions = {
  skipPreflight: true,
};

const PROTOCOLS = getProtocols(INPUT_TOKEN);

describe("borrow & deposit", () => {
  const provider = anchor.Provider.local();
  const userSigner = provider.wallet.publicKey;

  const client = new GoblinGold({
    connection: provider.connection,
    wallet: provider.wallet as anchor.Wallet,
  });

  const program = client.LendingArb;
  program.setToken(INPUT_TOKEN);

  const wrappedKeypair = anchor.web3.Keypair.generate();
  let userInputTokenAccount = wrappedKeypair.publicKey;
  let userLpTokenAccount: anchor.web3.PublicKey;

  it("Initialize vault", async () => {
    const tx = await program.initializeVault(
      new anchor.BN(0),
      BORROW_TOKEN_MINT
    );
    const txProtocols = await Promise.all(
      PROTOCOLS.map(async (protocol) =>
        program.methods
          .addProtocol(protocol)
          .accounts({
            userSigner,
            vaultAccount: program.vaultKeys[INPUT_TOKEN].vaultAccount,
          })
          .transaction()
      )
    );
    txProtocols.reduce((acc, txProtocol) => acc.add(txProtocol), tx);

    await program.provider.sendAndConfirm(tx, [], CONFIRM_OPTS);
    // const vaultData = await program.decodeVault();
    // const vaultProtocols = vaultData.protocols.map((data) => data.protocolId);

    // assert.deepStrictEqual(vaultProtocols, PROTOCOLS);
  });

  it("Initialize protocol accounts", async () => {
    const txs = await program.initializeProtocolAccounts();
    txs.map(async (tx) => {
      await program.provider.sendAndConfirm(tx, [], CONFIRM_OPTS);
    });
  });

  it("Set hashes", async () => {
    const txsHashes = await program.setHashes();
    const txHashes = txsHashes.reduce(
      (acc, tx) => acc.add(tx),
      new anchor.web3.Transaction()
    );
    await program.provider.sendAndConfirm(txHashes, [], CONFIRM_OPTS);
  });

  it("Deposit into vault", async () => {
    const amount = new anchor.BN(1_000_000_000);

    userLpTokenAccount = await spl.getAssociatedTokenAddress(
      program.vaultKeys[INPUT_TOKEN].vaultLpTokenMintAddress,
      userSigner,
      false
    );

    const tx = new anchor.web3.Transaction();

    if (INPUT_TOKEN === "WSOL") {
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
      userInputTokenAccount = await spl.getAssociatedTokenAddress(
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
      tx.add(await program.protocolDeposit());
      await program.provider.sendAndConfirm(tx, [], CONFIRM_OPTS);
    }
  });

  it("Borrow Solend", async () => {
    const tx = await program.borrow({
      userInputTokenAccount,
      userLpTokenAccount,
      amount: new anchor.BN(1000),
    });
    await program.provider.sendAndConfirm(tx, [], CONFIRM_OPTS);
  });

  xit("Deposit WSOL Francium", async () => {
    const tx = await program.borrow({
      userInputTokenAccount,
      userLpTokenAccount,
      amount: new anchor.BN(1000),
    });
  });
});
