import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { assert } from "chai";
import { GoblinGold, NetworkName } from "goblin-sdk";
import { BestApy } from "../target/types/best_apy";

describe("best_apy", () => {
  const testingProgram = anchor.workspace.BestApy as Program<BestApy>;

  const client = new GoblinGold(
    NetworkName.Mainnet,
    undefined,
    anchor.Provider.local(),
    testingProgram.programId
  );

  const program = client.BestApy;

  it("Initialize vault with weights", async () => {
    const protocolWeights = [2000, 2000, 2000, 2000, 2000];

    const txVault = await program.initializeVault();
    const txWeights = await program.setProtocolWeights(protocolWeights);
    txVault.add(txWeights);

    const txSigVault = await program.provider.send(txVault);
    console.log("tx init_vault:", txSigVault);

    const vaultData = await program.decodeVault();
    const vaultWeights = vaultData.protocols.map((data) => data.weight);

    assert.deepStrictEqual(vaultWeights, protocolWeights);
  });

  it("Initialize protocol accounts", async () => {
    const txsProtocols = await program.initializeProtocolAccounts();
    for (let i = 0; i < txsProtocols.length; ++i) {
      const txSig = await program.provider.send(txsProtocols[i]);
      console.log("tx init_protocols_" + i.toString() + ":", txSig);
    }
  });
});
