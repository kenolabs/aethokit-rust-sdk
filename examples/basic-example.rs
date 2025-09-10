use aethokit::{Aethokit, AethokitConfig, SponsorTxRequest};
use solana_sdk::{
    instruction::Instruction,
    message::Message,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use solana_client::rpc_client::RpcClient;
use serde::Serialize;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load your GAS KEY from environment
    let gas_key = std::env::var("AETHOKIT_GAS_KEY")
        .expect("set AETHOKIT_GAS_KEY in your environment");

    // initialize with config
    let aethokit_client = Aethokit::new(AethokitConfig {
        gas_key,
        rpc_or_network: Some("devnet".into()), // or None if you want default
    })?;

    // fetch gas address
    let sponsor_addr = aethokit_client.get_gas_address().await?;
    let sponsor_pubkey = Pubkey::from_str(&sponsor_addr)?;

    let rpc_url = "https://api.devnet.solana.com"; // or mainnet(https://api.mainnet-beta.solana.com)
    let rpc_client = RpcClient::new(rpc_url.to_string());

    // Sender (payer of funds, but not fees)
    let sender = Keypair::new();

    // Recipient
    let recipient =
        Pubkey::from_str("recipient")?;

    // Amount: 0.01 SOL
    let lamports = 10_000_000u64;

    // Create Transfer Instruction
    let instruction: Instruction =
        system_instruction::transfer(&sender.pubkey(), &recipient, lamports);

    // Recent blockhash (needed for transaction validity)
    let blockhash = rpc_client.get_latest_blockhash()?;

    // Message (sender pays, fee payer set to gas tank)
    let message = Message::new(&[instruction], Some(&sponsor_addr));

    let mut tx = Transaction::new_unsigned(message);
    tx.try_partial_sign(&[&sender], blockhash)?; // sender signs, fee payer not set

    // Serialize the tx (base64)
    let serialized_tx = base64::encode(tx.serialize());

    // sponsor the tx and get hash
    let hash = aethokit_client.sponsor_tx(serialized_tx).await?;
    println!("Hash: {}", hash);

    Ok(())
}
