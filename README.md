# Rust Blockchain Simulator
This is a simple blockchain simulator written in Rust. It demonstrates how blocks are created, linked, and hashed using SHA-256 with a basic proof-of-work mechanism.

## Features
* Custom Block struct with metadata

* Genesis block creation

* Chaining new blocks to the blockchain

* Simple proof-of-work using a nonce and SHA-256

* Logs all block data in a human-readable format

## How It Works
Each block contains:

* index: Block position in the chain

* data: Arbitrary string data (e.g., transaction info)

* hash: SHA-256 hash of the block

* previous: Hash of the previous block

* nonce: Value that satisfies the proof-of-work condition

* timestamp: Time of block creation (UTC)

## Hashing and Proof-of-Work
The hash is computed from the following string:

***{nonce}{index}{data}{timestamp}{previous_hash}***

A valid block must have a hash starting with **"000"**.

The find_nonce function increments the nonce until this condition is met. Every million attempts, it logs progress.

## Example Output
Running the program will generate output like:

```mathematica
Genesis Block
Block {
    index: 0,
    data: "Genesis Block",
    ...
}

Nonce: 1000000 / Hash: 29a7...

Block {
    index: 1,
    data: "Transaction Info",
    ...
}
```
...and so on for each new block.

# Usage
1. Install dependencies

Ensure you have Rust installed: https://www.rust-lang.org/tools/install

2. Clone the project and run it

```bash
$ cargo run
```

3. View the block data

Each block will be printed in debug format, showing its hash, data, nonce, and more.

# Dependencies
This project uses:

* chrono for date/time

* sha2 for SHA-256 hashing

# Possible Extensions
* Add block validation

* Store blockchain to a file

* Create a simple P2P network to distribute blocks

* Add a web interface to visualize the chain

# License
This project is licensed under the MIT License.