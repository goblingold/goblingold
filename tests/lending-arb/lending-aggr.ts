import * as anchor from "@project-serum/anchor";
import * as spl from "@solana/spl-token";
import { assert } from "chai";
import { GoblinGold, Protocols, TOKENS, decodeAccount } from "goblin-sdk-local";
import { getProtocols } from "./protocols";

const INPUT_TOKEN = "USDC";
const BORROW_TOKEN = "WSOL";
const CONFIRM_OPTS: anchor.web3.ConfirmOptions = {
  skipPreflight: true,
};

const PROTOCOLS = getProtocols(INPUT_TOKEN);


describe("deposit-from-native", () => {
  const provider = anchor.Provider.local();
  const userSigner = provider.wallet.publicKey;

  const client = new GoblinGold({
    connection: provider.connection,
    wallet: provider.wallet as anchor.Wallet,
  });

  const program = client.LendingAggr;


  it("Initialize vault", async () => {
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

    await program.provider.sendAndConfirm(tx, [], CONFIRM_OPTS);
    const vaultData = await program.decodeVault();
    const vaultProtocols = vaultData.protocols.map((data) => data.protocolId);

    assert.deepStrictEqual(vaultProtocols, PROTOCOLS);
  });



});
