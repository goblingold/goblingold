import * as anchor from "@project-serum/anchor";
import * as spl from "@solana/spl-token";
import { assert } from "chai";
import { GoblinGold, Protocols, TOKENS, decodeAccount } from "goblin-sdk-local";
import { getProtocols } from "./protocols";

const INPUT_TOKEN = "WSOL";
const CONFIRM_OPTS: anchor.web3.ConfirmOptions = {
  skipPreflight: true,
};

describe("deposit-from-native", () => {
  const provider = anchor.Provider.local();
  const userSigner = provider.wallet.publicKey;

  const client = new GoblinGold({
    connection: provider.connection,
    wallet: provider.wallet,
  });

  const program = client.BestApy;

  it("Initialize an empty vault", async () => {
    const tx = await program.initializeVault(new anchor.BN(0));
    await program.provider.sendAndConfirm(tx, [], CONFIRM_OPTS);
  });

  it("Deposit from native", async () => {
    const amount = new anchor.BN(1_000_000_000);

    const userLpTokenAccount = await spl.getAssociatedTokenAddress(
      program.vaultKeys[INPUT_TOKEN].vaultLpTokenMintAddress,
      userSigner,
      false
    );

    const wrappedKeypair = anchor.web3.Keypair.generate();
    const userWrappedAccount = wrappedKeypair.publicKey;
    const lamports = await spl.getMinimumBalanceForRentExemptAccount(
      program.provider.connection
    );

    const tx = new anchor.web3.Transaction()
      .add(
        anchor.web3.SystemProgram.createAccount({
          fromPubkey: userSigner,
          newAccountPubkey: userWrappedAccount,
          space: spl.ACCOUNT_SIZE,
          lamports,
          programId: spl.TOKEN_PROGRAM_ID,
        }),
        spl.createInitializeAccountInstruction(
          userWrappedAccount,
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
        await program.depositFromNative({
          userWrappedAccount,
          userLpTokenAccount,
          amount,
        })
      )
      .add(
        spl.createCloseAccountInstruction(
          userWrappedAccount,
          userSigner,
          userSigner,
          []
        )
      );
    await program.provider.sendAndConfirm(tx, [wrappedKeypair], CONFIRM_OPTS);

    const userLpTokenAccountInfo =
      await program.provider.connection.getAccountInfo(userLpTokenAccount);
    const data = decodeAccount(userLpTokenAccountInfo.data);
    const lpAmount = new anchor.BN(data.amount);

    assert.deepStrictEqual(amount, lpAmount);
  });
});
