import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { BestApy } from "../target/types/best_apy";

describe("best_apy", () => {
  anchor.setProvider(anchor.Provider.env());
  const program = anchor.workspace.BestApy as Program<BestApy>;

  it("Is initialized!", async () => {
    console.log("hello!");
  });
});
