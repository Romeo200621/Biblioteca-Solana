// No imports needed: web3, anchor, pg and more are globally available

describe("Bar", () => {
  it("crear bar", async () => {

    // Generar keypair para la cuenta del bar
    const barKp = new web3.Keypair();

    // Nombre del bar
    const nombre = "Bar Solana";

    // Enviar transacción para crear el bar
    const txHash = await pg.program.methods
      .crearBar(nombre)
      .accounts({
        bar: barKp.publicKey,
        owner: pg.wallet.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([barKp])
      .rpc();

    console.log(`Usa 'solana confirm -v ${txHash}' para ver los logs`);

    // Confirmar transacción
    await pg.connection.confirmTransaction(txHash);

    // Obtener la cuenta creada
    const bar = await pg.program.account.bar.fetch(barKp.publicKey);

    console.log("Datos del bar en blockchain:", bar);

    // Verificar que el nombre sea correcto
    assert.equal(bar.nombre, nombre);
  });
});
