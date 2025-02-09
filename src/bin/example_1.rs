use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_program::{message::Message, system_instruction};
use solana_sdk::{
    commitment_config::CommitmentConfig, native_token::LAMPORTS_PER_SOL, signature::Keypair,
    signer::Signer, transaction::Transaction,
};

fn main() -> Result<()> {
    let rpc_url = String::from("http://127.0.0.1:8899");
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let sender = create_and_fund_keypair(&client, LAMPORTS_PER_SOL)?;
    let receiver = Keypair::new();
    println!("from: {}", sender.pubkey());
    println!("to: {}", receiver.pubkey());

    let instruction =
        system_instruction::transfer(&sender.pubkey(), &receiver.pubkey(), 100_000_000);
    println!("\n{:#?}", instruction);

    let message = Message::new(&[instruction], Some(&sender.pubkey()));
    println!("\n{:#?}", message);

    let transaction = Transaction::new(&[&sender], message, client.get_latest_blockhash()?);
    println!("\n{:#?}", transaction);

    let signature = client.send_and_confirm_transaction(&transaction)?;
    println!(
        "\nTransaction: https://explorer.solana.com/tx/{}?cluster=custom",
        signature
    );
    Ok(())
}

pub fn create_and_fund_keypair(client: &RpcClient, lamports: u64) -> Result<Keypair> {
    let keypair = Keypair::new();
    let signature = client.request_airdrop(&keypair.pubkey(), lamports)?;
    loop {
        if client.confirm_transaction(&signature)? {
            break;
        }
    }
    Ok(keypair)
}
