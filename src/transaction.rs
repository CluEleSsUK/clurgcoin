use crate::key_pair::KeyPair;
use secp256k1::PublicKey;
use crate::crypto;

#[derive(Copy, Clone, Debug)]
pub struct Transaction {
    pub from: PublicKey,
    pub to: PublicKey,
    pub amount: i32,
}

impl Transaction {
    pub fn serialize(&self) -> String {
        return format!("{}{}{}", self.from, self.to, hex::encode(&format!("{}", self.amount)));
    }

    pub fn hash(&self) -> Vec<u8> {
        return crypto::sha256(self.serialize());
    }
}

#[derive(Debug)]
pub struct SignedTransaction {
    tx: Transaction,
    signature: String,
}

pub fn create_signed_transaction(keypair: &KeyPair, to: PublicKey, amount: i32) -> SignedTransaction {
    let tx = Transaction {
        from: keypair.public_key,
        to,
        amount,
    };
    let signature = keypair.sign(&tx.hash());
    return SignedTransaction { tx, signature: signature.to_string() };
}
