use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
pub struct Transaction {
    pub nonce: i32,
    pub fr: String,
    pub to: String,
    pub amount: i32,
}

impl Display for Transaction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Wallet {
    pub nonce: i32,
    pub address: String,
    pub balance: i32,
}

impl Wallet {
    pub fn new() -> Self {
        Wallet {
            nonce: 0,
            address: String::from("wallet_1"),
            balance: 300,
        }
    }

    pub fn create_transaction(&mut self, amount: i32) -> Transaction {
        let t: Transaction = Transaction {
            nonce: self.nonce,
            fr: String::from("wallet_2"),
            to: self.address.clone(),
            amount,
        };
        self.nonce += 1;
        t
    }
}
