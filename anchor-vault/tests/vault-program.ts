import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VaultProgram } from "../target/types/vault_program";
import { assert } from "chai";

describe("vault_program", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.VaultProgram as Program<VaultProgram>;
  const user = provider.wallet;

  let statePda: anchor.web3.PublicKey;
  let vaultPda: anchor.web3.PublicKey;
  let vaultBump: number;

  before(async () => {
    [statePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("state"), user.publicKey.toBuffer()],
      program.programId
    );

    [vaultPda, vaultBump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), statePda.toBuffer()],
      program.programId
    );
  });

  it("Initializes the vault", async () => {
    await program.methods
      .initialize()
      .accounts({
        user: user.publicKey,
        state: statePda,
        vault: vaultPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const state = await program.account.vaultState.fetch(statePda);
    assert.equal(state.amount.toNumber(), 0);
  });

  it("Deposits 1 SOL", async () => {
    const lamports = anchor.web3.LAMPORTS_PER_SOL;

    await program.methods
      .deposit(new anchor.BN(lamports))
      .accounts({
        signer: user.publicKey,
        vaultState: statePda,
        vault: vaultPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const state = await program.account.vaultState.fetch(statePda);
    assert.equal(state.amount.toNumber(), lamports);

    const vaultBalance = await provider.connection.getBalance(vaultPda);
    assert.equal(vaultBalance, lamports);
  });

  it("Withdraws 0.5 SOL", async () => {
    const withdrawAmount = anchor.web3.LAMPORTS_PER_SOL / 2;

    await program.methods
      .withdraw(new anchor.BN(withdrawAmount))
      .accounts({
        signer: user.publicKey,
        vaultState: statePda,
        vault: vaultPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const state = await program.account.vaultState.fetch(statePda);
    assert.equal(state.amount.toNumber(), withdrawAmount);

    const vaultBalance = await provider.connection.getBalance(vaultPda);
    assert.equal(vaultBalance, withdrawAmount);
  });

  it("Fails to withdraw more than available", async () => {
    const tooMuch = anchor.web3.LAMPORTS_PER_SOL;

    try {
      await program.methods
        .withdraw(new anchor.BN(tooMuch))
        .accounts({
          signer: user.publicKey,
          vaultState: statePda,
          vault: vaultPda,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      assert.fail("Should not allow withdrawing more than available");
    } catch (err) {
      const errMsg = err.toString();
      assert.include(errMsg, "Insufficient funds in vault");
    }
  });

  it("Closes the vault", async () => {
    const userBalanceBefore = await provider.connection.getBalance(user.publicKey);

    await program.methods
      .close()
      .accounts({
        user: user.publicKey,
        vaultState: statePda,
        vault: vaultPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    // PDA should no longer exist
    try {
      await program.account.vaultState.fetch(statePda);
      assert.fail("Vault state account should be closed");
    } catch (err) {
      assert.include(err.toString(), "Account does not exist");
    }

    const userBalanceAfter = await provider.connection.getBalance(user.publicKey);
    assert.isTrue(userBalanceAfter > userBalanceBefore - 1e6); 
  });
});
