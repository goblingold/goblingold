import * as anchor from "@project-serum/anchor";
import * as spl from "@solana/spl-token";
import { assert } from "chai";
import {
  GoblinGold,
  NetworkName,
  Protocols,
  TokenName,
  decodeAccount,
} from "goblin-sdk-local";
import { BestApy } from "../target/types/best_apy";

describe("rebalance", () => {
  const testingProgram = anchor.workspace.BestApy as anchor.Program<BestApy>;

  const client = new GoblinGold(
    NetworkName.Mainnet,
    undefined,
    anchor.Provider.local(),
    testingProgram.programId
  );

  const program = client.BestApy;
  const tokenInput = TokenName.WSOL;
  const userSigner = program.provider.wallet.publicKey;


  it("Refresh weights", async () => {

    const tx = await program.refreshWeights();
    const txSig = await program.provider.send(tx);
  });

  // it("Set zero weights", async () => {
  //  const protocolWeights = [0, 0, 0, 0, 10_000];
  //  const tx = await program.setProtocolWeights(protocolWeights);
  //  const txSig = await program.provider.send(tx);
  // });

  it("Rebalance from the protocols", async () => {
    const txs = await program.rebalance();
    console.log(txs.length, " xxxx")
    for (let i = 0; i < txs.length; ++i) {
      const txSig = await program.provider.send(txs[i], [], {
        skipPreflight: true,
      });
    }
  });
});
