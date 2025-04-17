import { AccountMeta, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, RpcResponseAndContext, sendAndConfirmTransaction, SignatureResult, Signer, Transaction, TransactionInstruction, TransactionSignature } from "@solana/web3.js";


export async function request_air_drop(connection : Connection, payer_public_key : PublicKey, funds_requested : number): Promise<RpcResponseAndContext<SignatureResult>>{

    const airdropTransactionSignature = await connection.requestAirdrop(
        payer_public_key,
        funds_requested,
    );
    return connection.confirmTransaction(airdropTransactionSignature);
}

export async function execute_properly_signed_transaction(connection : Connection, instructions :TransactionInstruction[], signers : Signer[])  {

    const transaction = new Transaction()

    let numTransactions = instructions.length

    for(let i = 0; i < numTransactions; i++){
        transaction.add(instructions[i])
    }

    return await sendAndConfirmTransaction(
        connection,
        transaction,
        signers,
    );
}


export async function get_balance(connection: Connection, payer_public_key: PublicKey){
    return connection.getBalance(payer_public_key);
}