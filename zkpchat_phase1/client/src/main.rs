use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce
};
use rand::RngCore;
use reqwest::Client;
use std::io::{self, Write};
use structopt::StructOpt;

/// A simple CLI to send and retrieve encrypted messages via relay/storage.
#[derive(StructOpt, Debug)]
#[structopt(name = "client")]
enum Cli {
    /// Send a message
    Send {
        /// The message to encrypt
        #[structopt(short, long)]
        message: String,

        /// Relay node URL
        #[structopt(long, default_value = "http://127.0.0.1:8081")]
        relay_url: String,

        /// Storage node URL
        #[structopt(long, default_value = "http://127.0.0.1:8082")]
        storage_url: String,
    },
    /// Retrieve messages from storage
    Retrieve {
        /// Storage node URL
        #[structopt(long, default_value = "http://127.0.0.1:8082")]
        storage_url: String,
    },
    /// Count messages in storage
    Count {
        /// Storage node URL
        #[structopt(long, default_value = "http://127.0.0.1:8082")]
        storage_url: String,
    },
    /// Get a specific message from storage
    Get {
        /// Storage node URL
        #[structopt(long, default_value = "http://127.0.0.1:8082")]
        storage_url: String,

        /// The index of the message to retrieve (0-based)
        #[structopt(long)]
        index: usize,
    },
    /// Delete messages from storage
    Delete {
        /// Storage node URL
        #[structopt(long, default_value = "http://127.0.0.1:8082")]
        storage_url: String,

        /// The index of the message to delete (optional, if not provided, deletes all messages)
        #[structopt(long)]
        index: Option<usize>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();
    let client = Client::new();

    match args {
        Cli::Send { message, relay_url, storage_url } => {
            println!("Encrypting message: {}", message);

            let mut rng_key = [0u8; 32];
            rand::thread_rng().fill_bytes(&mut rng_key);

            let key = Key::<Aes256Gcm>::from_slice(&rng_key);
            let cipher = Aes256Gcm::new(key);

            std::fs::write("key.txt", hex::encode(rng_key)).expect("Failed to save encryption key");
            println!("🔑 Encryption key saved as key.txt 🔑");

            // Nonce should be unique per message
            let mut nonce_bytes = [0u8; 12];
            rand::thread_rng().fill_bytes(&mut nonce_bytes);
            let nonce = Nonce::from_slice(&nonce_bytes);

            // Encrypt the message
            let ciphertext = cipher.encrypt(nonce, message.as_bytes())
                .expect("encryption failure!");

            // Combine nonce + ciphertext
            let mut payload = Vec::new();
            payload.extend_from_slice(&nonce_bytes);
            payload.extend_from_slice(&ciphertext);

            let encoded_payload = hex::encode(&payload);

            // Construct JSON to send to Relay
            let json_body = serde_json::json!({
                "encrypted_data": encoded_payload,
                "destination_url": storage_url,
            });

            // Send to Relay Node
            let resp = client
                .post(format!("{}/relay", relay_url))
                .json(&json_body)
                .send()
                .await?;

            println!("Relay response: {:?}", resp.text().await?);
        }

        Cli::Retrieve { storage_url } => {
            // Retrieve messages from Storage
            let resp = client
                .get(format!("{}/messages", storage_url))
                .send()
                .await?;

            let text = resp.text().await?;
            println!("Encrypted messages (JSON array): {}", text);

            // Parse and count messages
            let messages: serde_json::Value = serde_json::from_str(&text)?;
            let message_count = messages.as_array().map(|arr| arr.len()).unwrap_or(0);
            println!("Total messages stored: {}", message_count);
        }

        Cli::Count { storage_url } => {
            let resp = client
                .get(format!("{}/messages", storage_url))
                .send()
                .await?;

            let text = resp.text().await?;
            let messages: Vec<String> = serde_json::from_str(&text)?;

            println!("Total messages stored: {}", messages.len());
        }

        Cli::Get { storage_url, index } => {
            let resp = client
                .get(format!("{}/messages", storage_url))
                .send()
                .await?;

            let text = resp.text().await?;
            let messages: Vec<String> = serde_json::from_str(&text)?;

            if index < messages.len() {
                let encrypted_msg = &messages[index];
                println!("Retrieved Encrypted Message: {}", encrypted_msg);

                // Convert hex string back to bytes
                let payload_bytes = hex::decode(encrypted_msg).expect("Invalid hex encoding");

                // Extract nonce (first 12 bytes) and ciphertext (remaining)
                let (nonce_bytes, ciphertext) = payload_bytes.split_at(12);
                let nonce = Nonce::from_slice(nonce_bytes);

                // Attempt to read the key from `key.txt`
                let key_bytes = std::fs::read_to_string("key.txt")
                    .ok()
                    .and_then(|key_hex| hex::decode(key_hex.trim()).ok());

                let key = key_bytes.as_ref().map(|kb| Key::<Aes256Gcm>::from_slice(kb));

                // If key exists, attempt decryption
                if let Some(key) = key {
                    let cipher = Aes256Gcm::new(key);
                    match cipher.decrypt(nonce, ciphertext) {
                        Ok(plaintext) => {
                            let decrypted_msg = String::from_utf8(plaintext).expect("Invalid UTF-8");
                            println!("Decrypted Message: {}", decrypted_msg);
                        }
                        Err(_) => {
                            println!("⚠️  Decryption failed. Invalid key or corrupted data.");
                        }
                    }
                } else {
                    println!("⚠️  No decryption key found. Unable to decrypt the message.");
                }
            } else {
                println!("Invalid index: {}. Total messages: {}", index, messages.len());
            }
        }


        Cli::Delete { storage_url, index } => {
            if let Some(idx) = index {
                let resp = client
                    .delete(format!("{}/messages/{}", storage_url, idx))
                    .send()
                    .await?;

                println!("Deleted message {}. Response: {:?}", idx, resp.text().await?);
            } else {
                let resp = client
                    .delete(format!("{}/messages", storage_url))
                    .send()
                    .await?;

                println!("Deleted all messages. Response: {:?}", resp.text().await?);
            }
        }

    }

    Ok(())
}
