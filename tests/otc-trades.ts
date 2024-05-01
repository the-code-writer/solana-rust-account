describe("Test", () => {
  const userVaultAccount = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), pg.wallet.publicKey.toBuffer()],
    pg.program.programId
  )[0];

  const totalInteractionsAccount = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("counter"), pg.wallet.publicKey.toBuffer()],
    pg.program.programId
  )[0];

  it("Deposit into Vault", async () => {
    // Send transaction
    const amount = new BN(100000000);
    const depositTx = await pg.program.methods
      .deposit(amount)
      .accounts({
        userVaultAccount: userVaultAccount,
        signer: pg.wallet.publicKey,
        userInteractionsCounter: totalInteractionsAccount,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();

    // Confirm transaction
    await pg.connection.confirmTransaction(depositTx);

    // Fetch the created account
    const vaultData = await pg.program.account.userInteractions.fetch(
      totalInteractionsAccount
    );

    console.log("On-chain data is:", vaultData.totalDeposits);
  });

  it("Withdraw from vault", async () => {
    // Send transaction
    const amount = new BN(10000000);
    const depositTx = await pg.program.methods
      .withdraw(amount)
      .accounts({
        userVaultAccount: userVaultAccount,
        signer: pg.wallet.publicKey,
        userInteractionsCounter: totalInteractionsAccount,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([pg.wallet.keypair])
      .rpc();

    // Confirm transaction
    await pg.connection.confirmTransaction(depositTx);
  });
});