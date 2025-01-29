# zkpchat - Phase 1 (Early Alpha Version)

## Overview

zkpchat is a decentralized encrypted messaging system where messages can be securely relayed and stored without intermediate nodes having access to their contents. In this **Phase 1 Alpha Release**, the system supports:

- **Encryption on the client side** before sending messages.
- **Relay and storage nodes** that pass along and store encrypted messages without decrypting them.
- **Retrieval and decryption** of stored messages on the client side, demonstrating the importance of using the correct encryption key.

Each sent message generates a **new encryption key**, emphasizing the need for the correct key to successfully decrypt received messages. This is a demonstration of the Zero Knowledge Property, where only the sender and recipient with the correct key can decipher the message.

## Architecture

The system consists of three components:

1. **Client** - Encrypts messages, sends them to the relay, and retrieves messages from storage.
2. **Relay Node** - Receives encrypted messages and forwards them to the storage node.
3. **Storage Node** - Stores encrypted messages and allows retrieval of stored messages.

## Installation & Setup

### Prerequisites

- Rust (Latest stable version) - [Install Rust](https://www.rust-lang.org/tools/install)
- Cargo (Rust package manager, installed with Rust)

### Build the Project

1. Clone the repository:
   ```sh
   git clone https://github.com/yourusername/zkpchat.git
   cd zkpchat
   ```
2. Navigate to each component and build it:
   ```sh
   cd relay && cargo build --release
   cd ../storage && cargo build --release
   cd ../client && cargo build --release
   ```

## Running the Nodes

**Step 1:** Open a terminal and start the **Relay Node**:

```sh
cd relay
cargo run --release
```

**Step 2:** Open another terminal and start the **Storage Node**:

```sh
cd storage
cargo run --release
```

## Usage

Once the Relay and Storage nodes are running, open a third terminal for the client and use the following commands:

### **1. Sending an Encrypted Message**

Each time a message is sent, a new encryption key is generated. This key is stored in `key.txt`, and without it, the message cannot be decrypted.

```sh
cd client
cargo run --release -- send --message "Hello from Phase 1" --relay-url http://127.0.0.1:8081 --storage-url http://127.0.0.1:8082
```

**Expected Output:**

```
Encrypting message: Hello from Phase 1
🔑 Encryption key saved as key.txt 🔑
Relay response: "Relayed to storage node successfully"
```

> **Note:** Each message generates a new key. To decrypt a specific message, you must have the correct key from when it was sent.

### **2. Counting Stored Messages**

```sh
cargo run --release -- count --storage-url http://127.0.0.1:8082
```

### **3. Retrieving All Messages (Encrypted)**

```sh
cargo run --release -- retrieve --storage-url http://127.0.0.1:8082
```

### **4. Retrieving a Specific Message (With Attempted Decryption)**

```sh
cargo run --release -- get --storage-url http://127.0.0.1:8082 --index 0
```

- If `key.txt` exists and matches the encryption key used, the message will be decrypted.
- If the key does not match or is missing, an error will be displayed.

### **5. Deleting Messages**

- Delete a specific message:
  ```sh
  cargo run --release -- delete --storage-url http://127.0.0.1:8082 --index 0
  ```
- Delete all messages:
  ```sh
  cargo run --release -- delete --storage-url http://127.0.0.1:8082
  ```

## Next Steps, Enhancements, & Future Improvements
### Phase 1: Minimum Viable Network (Testnet)
- Persistent Storage:
  - Replace the in-memory Vec<String> with a real database (e.g., sled or RocksDB).
  - For each message, store (encrypted_data, message_id, timestamp, sender, etc.).
- Key Management:
  - Instead of generating a random key per message, have each user generate a stable keypair or store a symmetric key.
  - Integrate with an Aleo-based Identity-based encryption (IBP) in future phases.
  - Persistent encryption keys for sender/recipient to allow long-term decryption.
  - Secure transport of encryption keys for designated recipients.
- Network Protocol (p2p):
  - For true decentralization, replace HTTP with a p2p library like libp2p or use direct TCP/UDP sockets.
  - Implement NAT traversal, node discovery, etc.
### Phase 2:  Integrate Zero-Knowledge Proofs (ZKPs) with Aleo
- Aleo Smart Contracts:
  - Deploy minimal contracts that store a user’s public key or a hash of it.
  - Possibly store commitments referencing each message ID for integrity checks.
- Computation Node:
  - Build a Rust microservice that generates a basic zero-knowledge proof (e.g., “I own the key for address X”).
  - Integrate with the relay/storage to verify user requests.
- Zero-Knowledge:
  - Embed Aleo-based ZK logic for verifying ownership,
  - Ensuring data integrity,
  - Providing on-chain receipts of message existence—without revealing the content.
### Phase 3: Full Decentralization & Node Discovery
- Node Discovery Protocol:
  - Implement a peer-to-peer discovery mechanism (e.g., via libp2p) so that new nodes can join the network and register as relay, storage, or computation providers.
- Consensus:
  - Rely on Aleo’s blockchain for finality of identity and proofs, but also define how the network ensures correct routing and data availability off-chain.
### Phase 4: Incentives, GUI, and Web Interface
- Webmail:
  - Implement a basic web interface that calls your relay node APIs, handles encryption in-browser, and interacts with the storage layer.
- Enhanced Storage:
  - Add replication or partitioning to scale read/write requests.
- Security Hardening:
  - Add authentication flows with ZK proofs (e.g., login with Aleo-based key).
- Incentives:
  - Introduce tokens to reward for node operators.
  - Create a new smart contract and token for node services.
### Phase 5: Email, Scaling & Advanced Features
- Performance Tuning:
  - Optimize encryption, ZK proof generation, caching layers.
  - Investigate incremental proofs or batched verification if usage grows.
- Premium Features:
  - Offload heavy computations (e.g., large-file encryption or advanced proofs) to dedicated computation nodes with billing.
- Advanced Email Protocol:
  - Incorporate standard email interoperability (SMTP/IMAP gateways) for compatibility with the existing email ecosystem.
- Mobile Clients:
  - Develop robust Rust-based or cross-platform clients with push notifications, offline caching, key management, etc.



## License

MIT License

## Contributing

Contributions are welcome! Feel free to submit issues and pull requests to improve the system.

---

This Phase 1 implementation is an **early alpha version** aimed at demonstrating encrypted messaging workflows with a basic relay/storage system. Thank you for testing zkpchat!
