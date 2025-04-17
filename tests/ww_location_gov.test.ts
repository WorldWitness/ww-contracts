import { Buffer } from "buffer";
import { WwLocation } from "../target/types/ww_location";
import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, sendAndConfirmTransaction, Transaction } from "@solana/web3.js";
import * as general from './general'
import * as ww_location from './programs/ww_location'
describe("ww_location", () => {
  // Configure the client to use the local cluster.

  const payer = Keypair.generate();
  const connection = new Connection("http://127.0.0.1:8899", "confirmed");

   
  it("is initialized and enabled!", async () => {
    const airdropTransactionSignature = await general.request_air_drop(connection, payer.publicKey, LAMPORTS_PER_SOL);
    await general.get_balance(connection, payer.publicKey);
    

    const initializeInstruction = await ww_location.create_initialize_instruction(payer.publicKey);
    await general.execute_properly_signed_transaction(connection, [initializeInstruction], [payer]);

    const locationCounterPda = await PublicKey.findProgramAddress(
      [Buffer.from("location_counter")],
      ww_location.program.programId
    );
    let counterAccount  = await ww_location.get_location_counter(locationCounterPda[0]);

    expect(counterAccount.isFrozen).toBeTruthy();

    const enableInstruction = await ww_location.create_enable_location_instruction(payer.publicKey, locationCounterPda[0]);
    await general.execute_properly_signed_transaction(connection, [enableInstruction], [payer]);

    counterAccount = await ww_location.program.account.locationCounter.fetch(locationCounterPda[0]) ;
    expect(counterAccount.isFrozen).toBeFalsy();
  });

  
});
