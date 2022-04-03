import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
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

  it("Is initialized!", async () => {
    const protocolWeights = [2000, 2000, 2000, 2000, 2000];

    const txVault = await program.initializeVault();
    const txWeights = await program.setProtocolWeights(protocolWeights);
    txVault.add(txWeights);

    const txSigVault = await program.provider.send(txVault);
    console.log("tx init_vault:", txSigVault);
  });
});
