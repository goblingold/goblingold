import * as anchor from "@project-serum/anchor";
import * as spl from "@solana/spl-token";
import { assert } from "chai";
import { GoblinGold, Protocols, TOKENS, decodeAccount } from "goblin-sdk-local";

const INPUT_TOKEN = "WSOL";
const INPUT_TOKEN_MINT = new anchor.web3.PublicKey(
  TOKENS[INPUT_TOKEN].mintAddress
);

const PROTOCOLS = [Protocols.Solend, Protocols.Tulip, Protocols.Francium];

const WEIGHTS_SCALE = 10_000;
const WEIGHTS = PROTOCOLS.map((_, indx) => {
  const len = PROTOCOLS.length;
  const weight = Math.floor(WEIGHTS_SCALE / len);
  return indx === 0 ? WEIGHTS_SCALE - weight * (len - 1) : weight;
});

const CONFIRM_OPTS: anchor.web3.ConfirmOptions = {
  skipPreflight: true,
};

describe("withdraw-ticket", () => {
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
    tx.add(await program.setProtocolWeights(WEIGHTS));

    await program.provider.sendAndConfirm(tx, [], CONFIRM_OPTS);
    const vaultData = await program.decodeVault();
    const vaultWeights = vaultData.protocols.map((data) => data.weight);
    const vaultProtocols = vaultData.protocols.map((data) => data.protocolId);

    assert.deepStrictEqual(vaultWeights, WEIGHTS);
    assert.deepStrictEqual(vaultProtocols, PROTOCOLS);
  });

  it("Initialize withdraw ticket mint", async () => {
    const tx = await program.initializeTicketMint();
    await program.provider.sendAndConfirm(tx, [], CONFIRM_OPTS);
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

    const wrappedKeypair = anchor.web3.Keypair.generate();
    const userInputTokenAccount = wrappedKeypair.publicKey;
    const lamports = await spl.getMinimumBalanceForRentExemptAccount(
      program.provider.connection
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
  });

  it("Deposit into the protocols", async () => {
    const [_txsWithdraw, txsDeposit] = await program.rebalance();
    await Promise.all(
      txsDeposit.map(async (tx) =>
        program.provider.sendAndConfirm(tx, [], CONFIRM_OPTS)
      )
    );
  });

  it("Open withdraw ticket", async () => {
    const userLpTokenAccount = await spl.getAssociatedTokenAddress(
      program.vaultKeys[INPUT_TOKEN].vaultLpTokenMintAddress,
      userSigner,
      false
    );

    const [vaultUserTicketAccount, _bump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from("ticket_mint"),
          program.vaultKeys[INPUT_TOKEN].vaultTicketMintPubkey.toBuffer(),
          userSigner.toBuffer(),
        ],
        program.programId
      );

    const [userLpTokenAccountInfo, vaultUserTicketAccountInfo] =
      await anchor.utils.rpc.getMultipleAccounts(program.provider.connection, [
        userLpTokenAccount,
        vaultUserTicketAccount,
      ]);

    if (!userLpTokenAccountInfo) {
      throw new Error("Error: user_lp_token_account not found");
    }

    const data = decodeAccount(userLpTokenAccountInfo.account.data);
    const lpAmount = new anchor.BN(data.amount);

    const tx = new anchor.web3.Transaction();

    if (!vaultUserTicketAccountInfo) {
      tx.add(await program.createVaultUserTicketAccount({userTicketAccountOwner:userSigner}));
    }

    tx.add(
      await program.openWithdrawTicket({
        userLpTokenAccount,
        lpAmount,
      })
    );
    await program.provider.sendAndConfirm(tx, [], CONFIRM_OPTS);
  });

  it("Close withdraw ticket", async () => {
    const [vaultUserTicketAccount, _bump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from("ticket_mint"),
          program.vaultKeys[INPUT_TOKEN].vaultTicketMintPubkey.toBuffer(),
          userSigner.toBuffer(),
        ],
        program.programId
      );

    const vaultUserTicketAccountInfo =
      await program.provider.connection.getAccountInfo(vaultUserTicketAccount);
    if (!vaultUserTicketAccountInfo) {
      throw new Error("Error: vault_user_ticket_account_info not found");
    }

    const data = decodeAccount(vaultUserTicketAccountInfo.data);
    const lpAmount = new anchor.BN(data.amount);

    const wrappedKeypair = anchor.web3.Keypair.generate();
    const userInputTokenAccount = wrappedKeypair.publicKey;
    const lamports = await spl.getMinimumBalanceForRentExemptAccount(
      program.provider.connection
    );

    const txs = await program.closeWithdrawTicket({
      userInputTokenAccount,
      lpAmount,
    });

    await Promise.all(
      txs.map(async (tx) => {
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

        return program.provider.sendAndConfirm(
          txAll,
          [wrappedKeypair],
          CONFIRM_OPTS
        );
      })
    );
  });
});
