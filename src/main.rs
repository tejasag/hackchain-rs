extern crate crypto;
extern crate itertools;

pub mod block_factory;
pub mod hash;
pub mod merkel_tree;
pub mod wallet;

use block_factory::*;
use hash::*;
use wallet::*;

// use merkel_tree::*;

fn main() {
    let mut factory = BlockFactory::new();
    for _i in 1..3 {
        let mut wallet = Wallet::new();
        let transactions = vec![1, 2]
            .iter()
            .map(|e| wallet.create_transaction(e * 100))
            .collect::<Vec<Transaction>>();

        factory.create_block(
            String::from("random data"),
            String::from("tejas"),
            transactions,
        );
        println!("{:#?}", factory.last_block.as_ref().unwrap());
        println!("{}", hash(factory.last_block.as_ref().unwrap().to_string()));
    }
}
