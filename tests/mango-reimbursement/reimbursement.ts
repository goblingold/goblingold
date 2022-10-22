import * as anchor from "@project-serum/anchor";
import * as spl from "@solana/spl-token";
import { assert } from "chai";
import { GoblinGold, Protocols, TOKENS, decodeAccount } from "goblin-sdk-local";
import { BN, Idl, web3 } from "@project-serum/anchor";

const INPUT_TOKEN = "USDC";
const CONFIRM_OPTS: anchor.web3.ConfirmOptions = {
  skipPreflight: true,
};

describe("mango reimbursement", () => {
  const provider = anchor.Provider.local();
  const userSigner = provider.wallet.publicKey;

  const client = new GoblinGold({
    connection: provider.connection,
    wallet: provider.wallet,
  });

  const program = client.BestApy;

  program.setToken(INPUT_TOKEN);

  it("Mango reimbursement", async () => {
    const wrappedKeypair = anchor.web3.Keypair.generate();
    const userWrappedAccount = wrappedKeypair.publicKey;
    const lamports = await spl.getMinimumBalanceForRentExemptAccount(
      program.provider.connection
    );

    let tx = await program.mangoReimbursement(
      new anchor.web3.PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
      15,
      new BN(1133)
    );

    await program.provider.sendAndConfirm(tx, [], CONFIRM_OPTS);

    await sleep(2000);
  });
});

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
