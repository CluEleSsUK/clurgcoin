mod crypto;
mod block;
mod transaction;
mod key_pair;

use crate::transaction::{SignedTransaction, create_signed_transaction};
use crate::key_pair::create_key_pair;

fn main() {
    let bob = create_key_pair();
    let alice = create_key_pair();

    let tx1 = create_signed_transaction(&bob, alice.public_key, 10);
    let tx2 = create_signed_transaction(&alice, bob.public_key, 5);

    let transactions: Vec<SignedTransaction> = vec![tx1, tx2];
    let proposed_block = block::ProposedBlock { transactions };

    println!("Mining block with {} txs", proposed_block.transactions.len());
    let block = proposed_block.mine(3);
    println!("Mined block: {:#?}", block);
}

