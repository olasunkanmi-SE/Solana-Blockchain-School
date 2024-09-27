import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { School } from "../target/types/school";

describe("school", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.School as Program<School>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
