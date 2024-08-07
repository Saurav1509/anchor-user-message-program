import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorUserMessage } from "../target/types/anchor_user_message";
import { expect } from "chai";

describe("anchor-user-message", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.AnchorUserMessage as Program<AnchorUserMessage>;

  const user = {
    name: "Saurav",
    message: "Hi this is Saurav."
  }

  const [userPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(user.name), provider.wallet.publicKey.toBuffer()],
    program.programId,
  )

  it("Added User Message!", async () => {
    // Add your test here.
    const tx = await program.methods
      .addUserMessage(user.name, user.message)
      .rpc()
    
    const account = await program.account.userMessageState.fetch(userPda);
    expect(user.name === account.name);
    expect(user.message === account.message);
  });

  it("Updated User Message!", async () => {
    // Add your test here.

    const newMessage = "This is the udpated message"

    const tx = await program.methods
      .updateUserMessage(user.name, user.message)
      .rpc()
    
    const account = await program.account.userMessageState.fetch(userPda);
    expect(user.name === account.name);
    expect(newMessage === account.message);
  });

  it("Deleted User Message!", async () => {
    // Add your test here.

    const tx = await program.methods
      .deleteUserMessage(user.name)
      .rpc()
  });
});
