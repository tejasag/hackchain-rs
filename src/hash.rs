use super::crypto::{hmac::Hmac, mac::Mac, sha2::Sha256};
use crate::itertools::Itertools;

const SALT: &[u8] = b"some";

pub fn hash(thing: String) -> String {
    let mut algo = Hmac::new(Sha256::new(), SALT);

    algo.input(thing.as_str().as_bytes());
    algo.result()
        .code()
        .iter()
        .format_with("", |byte, f| f(&format_args!("{:02x}", byte)))
        .to_string()
}
