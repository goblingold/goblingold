import * as anchor from "@project-serum/anchor";
import { GoblinGold } from "goblin-sdk-local";
import { BN } from "@project-serum/anchor";

const INPUT_TOKEN = "MNGO";
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

  it("Mango reimbursement mngo", async () => {
    let tx = await program.mangoReimbursement(
      0,
      new BN(12090) //12563
    );

    await program.provider.sendAndConfirm(tx, [], CONFIRM_OPTS);
  });
});
