use super::{hash::hash, merkel_tree::merkel_tree, wallet::Transaction};
use std::fmt::{self, Debug, Display, Formatter};

#[derive(Debug, Clone)]
pub struct Block {
    pub height: i32,
    pub data: String,
    pub timestamp: String,
    pub miner: String,
    pub prev_block: Option<Box<Block>>,
    pub prev_hash: Option<String>,
    pub transactions: Vec<Transaction>,
    pub merkel_root: Option<String>,
}

impl Block {
    pub fn new(
        height: i32,
        data: String,
        timestamp: String,
        miner: String,
        prev_block: Option<Box<Block>>,
        prev_hash: Option<String>,
        transactions: Vec<Transaction>,
    ) -> Self {
        let mut b = Block {
            height,
            data,
            timestamp,
            miner,
            prev_block,
            prev_hash,
            transactions,
            merkel_root: None,
        };

        // Self::calculate_merkel_root();
        b.calculate_merkel_root();
        b
    }

    pub fn calculate_merkel_root(&mut self) {
        let t = self
            .transactions
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>();
        self.merkel_root = Some(merkel_tree(t));
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/*
impl Block {
    pub fn calc_merkel_root
}
*/

pub struct BlockFactory {
    pub height: i32,
    pub last_block: Option<Block>,
}

impl BlockFactory {
    pub fn new() -> Self {
        BlockFactory {
            height: 0,
            last_block: None,
        }
    }

    pub fn create_block(&mut self, data: String, miner: String, transactions: Vec<Transaction>) {
        let prev_block: Option<Box<Block>> = match &self.last_block {
            Some(e) => Some(Box::new(e.to_owned())),
            None => None,
        };
        let prev_hash: Option<String> = match prev_block {
            Some(_) => Some(hash(prev_block.as_ref().unwrap().to_string())),
            None => None,
        };
        self.height += 1;
        self.last_block = Some(Block::new(
            self.height,
            data,
            String::from("1st Jan 1970"),
            miner,
            prev_block,
            prev_hash,
            transactions,
        ));
    }
}
