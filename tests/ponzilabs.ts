import assert from "assert";
import * as anchor from "@project-serum/anchor";
import * as spl from "@solana/spl-token";
import idl from "../target/idl/best_apy.json";
import * as client from "wen-token-client";

const ForbiddenRefresh =
  "Can not refresh weights yet, not enough slots since last refresh";

const E3 = 1000;

function printVault(vault) {
  console.log("-------------- vault");
  for (const i in vault.weights) {
    console.log(i, "deposited   ", vault.protocolDeposited[i].toNumber());
    console.log(i, "protocol tvl", vault.protocolTvl[i].toNumber());
    console.log(i, "weights     ", vault.weights[i]);
  }
  console.log("  TOTAL AMOUNT", vault.totalAmount.toNumber());
  console.log("--------------------");
}

process.env.ANCHOR_WALLET = process.env.HOME + "/.config/solana/id.json";

describe("springboard", () => {
  const program = new client.GoblinGold().BestApy;
  const userSigner = program.provider.wallet.publicKey;

  // Will be updated on init_vault accounts
  let vaultLpToken;
  const amount = new anchor.BN(0.001 * anchor.web3.LAMPORTS_PER_SOL);
  // Mint some native tokens to the user
  let userInputTokenAccount;

  xit("init_vault accounts", async () => {
    let vaultAccount = anchor.web3.Keypair.generate();
    let tx = new anchor.web3.Transaction();
    tx.add(await program.initializeVault(vaultAccount.publicKey));
    tx.add(await program.initializeProtocolAccounts());
    const txsAll = await program.provider.send(tx, [vaultAccount]);
    console.log("tx init:", txsAll);
  });

  it("initiate constants", async () => {
    userInputTokenAccount = await spl.Token.createWrappedNativeAccount(
      program.provider.connection,
      spl.TOKEN_PROGRAM_ID,
      userSigner,
      program.provider.wallet.payer,
      amount.toNumber()
    );
    vaultLpToken = new spl.Token(
      program.provider.connection,
      new anchor.web3.PublicKey(program.vaultKeys.vaultLpTokenMintAddress),
      spl.TOKEN_PROGRAM_ID,
      //@ts-ignore
      program.provider.wallet.payer
    );
    printVault(await program.decodeVault());
  });

  xit("deposit into vault", async () => {
    const userLpTokenAccount =
      await vaultLpToken.getOrCreateAssociatedAccountInfo(userSigner);

    // Balances before the deposit
    let userLpTokenBalanceBefore = (
      await vaultLpToken.getAccountInfo(userLpTokenAccount.address)
    ).amount.toNumber();

    const tx = await program.deposit({
      userInputTokenAccount,
      userLpTokenAccount: userLpTokenAccount.address,
      amount,
    });
    const txsAll = await program.provider.send(tx);

    console.log("deposit tx:", txsAll);

    printVault(await program.decodeVault());

    // Balances after the deposit
    let userLpTokenBalanceAfter = (
      await vaultLpToken.getAccountInfo(userLpTokenAccount.address)
    ).amount.toNumber();

    assert.ok(
      userLpTokenBalanceBefore + amount.toNumber() == userLpTokenBalanceAfter
    );
  });

  it("withdraw from vault", async () => {
    const lpAmount = new anchor.BN(1330330);

    // Get the user associated input and vault token accounts
    let userLpTokenAccount =
      await vaultLpToken.getOrCreateAssociatedAccountInfo(userSigner);

    // Balances before the withdraw
    let userLpTokenBalanceBefore = userLpTokenAccount.amount.toNumber();

    const tx = await program.withdraw({
      userInputTokenAccount,
      userLpTokenAccount: userLpTokenAccount.address,
      lpAmount,
    });
    const txsAll = await program.provider.send(tx);
    console.log("withdraw tx:", txsAll);

    // Balances after the withdraw
    let userLpTokenBalanceAfter = (
      await vaultLpToken.getOrCreateAssociatedAccountInfo(userSigner)
    ).amount.toNumber();

    assert.ok(
      userLpTokenBalanceBefore - lpAmount.toNumber() == userLpTokenBalanceAfter
    );
    printVault(await program.decodeVault());
  });
  return;
  it("refresh rewards", async () => {
    printVault(await program.decodeVault());

    const tx = await program.refreshRewards();
    const txsAll = await program.provider.send(tx);
    console.log("refresh rewards tx:", txsAll);
    printVault(await program.decodeVault());
  });

  xit("fail to try to refresh again ", async () => {
    try {
      const tx = await program.refreshRewards();
      const txsAll = await program.provider.send(tx);
      assert.ok(false);
    } catch (err: any) {
      assert.equal(ForbiddenRefresh, err.msg);
    }
  });

  it("deposit into protocols", async () => {
    const txs = await program.depositProtocols();
    txs.forEach(async (tx) => {
      const txSig = await program.provider.send(tx);
      console.log("deposit into protocols tx:", txSig);
    });

    printVault(await program.decodeVault());
  });

  it("withdraw from protocol", async () => {
    const lpAmount = new anchor.BN(1 * E3);

    let userLpTokenAccount =
      await vaultLpToken.getOrCreateAssociatedAccountInfo(userSigner);

    const tx = await program.withdraw({
      userInputTokenAccount,
      userLpTokenAccount: userLpTokenAccount.address,
      lpAmount,
    });
    const txsAll = await program.provider.send(tx);
    console.log("withdraw tx:", txsAll);

    printVault(await program.decodeVault());
  });
});
