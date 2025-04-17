import { Program } from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import { PublicKey, TransactionInstruction } from "@solana/web3.js";
import { WwLocation } from "../../target/types/ww_location";




export const program = anchor.workspace.ww_location as Program<WwLocation>;

export async function create_initialize_instruction(payerPublicKey: PublicKey) : Promise<TransactionInstruction>  {
    return program.methods
    .initialize()
    .accounts({
      payer: payerPublicKey,
      })
    .instruction();
}

export async function create_enable_location_instruction(payerPublicKey: PublicKey, locationCounterPda : PublicKey) {

    return await program.methods
    .enableLocationCreation()
    .accounts({
      payer: payerPublicKey,
      locationCounter: locationCounterPda,
      })
    .instruction();
}

export async function get_location_counter(public_key : PublicKey): Promise<any> {
      return program.account.locationCounter.fetch(public_key)
}