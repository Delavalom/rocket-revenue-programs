import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolanaFacebook } from "../target/types/solana_facebook";

describe("solana-facebook", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolanaFacebook as Program<SolanaFacebook>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
