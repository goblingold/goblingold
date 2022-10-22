import * as anchor from "@project-serum/anchor";
import * as spl from "@solana/spl-token";
import { assert } from "chai";
import { GoblinGold, Protocols, TOKENS, decodeAccount } from "goblin-sdk-local";
import { BN, Idl, web3 } from "@project-serum/anchor";
import { Provider } from "@project-serum/anchor";

const INPUT_TOKEN = "USDC";
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

  program.setToken(INPUT_TOKEN);

  it("Mango reimbursement", async () => {
    let tx = await program.mangoReimbursement(15, new BN(3529));

    let sig = await program.provider.sendAndConfirm(tx, [], CONFIRM_OPTS);
    console.log("reimbursement tx ", sig.toString());
  });
});
