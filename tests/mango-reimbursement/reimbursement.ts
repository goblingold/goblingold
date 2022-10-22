import * as anchor from "@project-serum/anchor";
import { BN, Idl, web3 } from "@project-serum/anchor";
import { Provider } from "@project-serum/anchor";
import * as spl from "@solana/spl-token";
import { assert } from "chai";
import { GoblinGold, Protocols, TOKENS, decodeAccount } from "goblin-sdk-local";

const CONFIRM_OPTS: anchor.web3.ConfirmOptions = {
  skipPreflight: true,
};

describe("mango reimbursement", () => {
  const provider = anchor.Provider.local();

  const client = new GoblinGold({
    connection: provider.connection,
    wallet: provider.wallet,
  });

  const program = client.BestApy;

  xit("Mango reimbursement USDC", async () => {
    program.setToken("USDC");
    let tx = await program.mangoReimbursement(15, new BN(3529));
    await program.provider.sendAndConfirm(tx, [], CONFIRM_OPTS);
  });

  xit("Mango reimbursement WSOL", async () => {
    program.setToken("WSOL");
    let tx = await program.mangoReimbursement(3, new BN(2002));
    await program.provider.sendAndConfirm(tx, [], CONFIRM_OPTS);
  });

  xit("Mango reimbursement MNGO", async () => {
    program.setToken("MNGO");
    let tx = await program.mangoReimbursement(0, new BN(12090));
    await program.provider.sendAndConfirm(tx, [], CONFIRM_OPTS);
  });

  xit("Mango reimbursement RAY", async () => {
    program.setToken("RAY");
    let tx = await program.mangoReimbursement(6, new BN(0));
    await program.provider.sendAndConfirm(tx, [], CONFIRM_OPTS);
  });

  xit("Mango reimbursement mSOL", async () => {
    program.setToken("mSOL");
    let tx = await program.mangoReimbursement(10, new BN(0));
    await program.provider.sendAndConfirm(tx, [], CONFIRM_OPTS);
  });

  it("Mango reimbursement USDT", async () => {
    program.setToken("USDT");
    let tx = await program.mangoReimbursement(4, new BN(0));
    await program.provider.sendAndConfirm(tx, [], CONFIRM_OPTS);
  });
});
