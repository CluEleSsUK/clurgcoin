use hex;

use crate::transaction::SignedTransaction;
use crate::crypto;

#[derive(Debug)]
pub struct ProposedBlock {
    pub transactions: Vec<SignedTransaction>
}

#[derive(Debug)]
pub struct Block {
    transactions: Vec<SignedTransaction>,
    nonce: u32,
    hash: String,
}

impl ProposedBlock {
    pub fn mine(self, difficulty: usize) -> Block {
        let mut nonce: u32 = 0;
        let mut block_hash = String::new();

        while !block_hash.starts_with(&"0".repeat(difficulty)) {
            let block = format!("{}{}", block_hash, nonce);
            block_hash = hex::encode(crypto::sha256(block.clone()));
            nonce += 1;
        }

        return Block {
            hash: block_hash,
            nonce,
            transactions: self.transactions,
        };
    }
}
