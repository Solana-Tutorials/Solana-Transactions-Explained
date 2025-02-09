// 1. Create one or more Instructions
// 2. Create a Message (with instructions)
// 3. Create a Transaction (with message, and signatures)
// 4. Send the Transaction

// Think of Transactions as envelopes.
// Think of Instructions as forms we fill out and put in the envelopes.

use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_program::message::Message;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    native_token::LAMPORTS_PER_SOL,
    signature::Keypair,
    signer::Signer,
    system_program,
    transaction::Transaction,
};

fn main() -> Result<()> {
    let rpc_url = String::from("http://127.0.0.1:8899");
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let sender = create_and_fund_keypair(&client, LAMPORTS_PER_SOL)?;
    let receiver = Keypair::new();
    println!("from: {}", sender.pubkey());
    println!("to: {}", receiver.pubkey());

    // Instruction
    let program_id = system_program::id();
    let accounts = vec![
        AccountMeta::new(sender.pubkey(), true),
        AccountMeta::new(receiver.pubkey(), false),
    ];

    let transfer_instruction_index: u32 = 2;
    let lamports: u64 = 100_000_000;

    let mut data = [0u8; 12];
    let (instruction_index_bytes, lamports_bytes) = data.split_at_mut(4);
    instruction_index_bytes.copy_from_slice(&transfer_instruction_index.to_le_bytes());
    lamports_bytes.copy_from_slice(&lamports.to_le_bytes());

    let instruction = Instruction::new_with_bytes(program_id, &data, accounts);
    println!("\n{:#?}", instruction);

    // Message
    let message = Message::new_with_blockhash(
        &[instruction],
        Some(&sender.pubkey()),
        &client.get_latest_blockhash()?,
    );
    println!("\n{:#?}", message);

    // Transaction
    let mut tx = Transaction::new_unsigned(message);
    tx.sign(&[&sender], tx.message.recent_blockhash);
    print!("\n{:#?}", tx);

    let signature = client.send_and_confirm_transaction(&tx)?;
    println!(
        "\nTransaction: https://explorer.solana.com/tx/{}?cluster=custom",
        signature
    );

    Ok(())
}

pub fn create_and_fund_keypair(client: &RpcClient, lamports: u64) -> Result<Keypair> {
    // Create a new Keypair
    let keypair = Keypair::new();

    // Request an airdrop
    let signature = client.request_airdrop(&keypair.pubkey(), lamports)?;

    // Wait for transaction confirmation
    loop {
        if client.confirm_transaction(&signature)? {
            break;
        }
    }

    // Return the Keypair
    Ok(keypair)
}
