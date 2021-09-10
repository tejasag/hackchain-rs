use super::hash::hash;
use std::convert::TryInto;

pub fn merkel_tree(items: Vec<String>) -> String {
    // let mut t: Vec<String> = items.iter().map(|e| e.to_string()).collect::<Vec<String>>();
    let mut t = items;
    while t.len() != 1 {
        if t.len() % 2 != 0 {
            t.push(t.last().unwrap().to_owned());
        }

        let mut temp_arr: Vec<String> = vec![];
        let mut i: usize = 0;

        while i < t.len().try_into().unwrap() {
            let hash1: String = hash(t[i].to_string());
            let hash2: String = hash(t[i + 1].to_string());
            let leaf = format!("{}{}", hash1, hash2);
            temp_arr.push(leaf);
            i += 2;
        }

        t = temp_arr
    }

    t[0].to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_1() {
        use crate::{hash::hash, merkel_tree::merkel_tree};

        let opts = vec![String::from("a"), String::from("b")];
        let hash1: String = hash(String::from("a"));
        let hash2: String = hash(String::from("b"));
        let leaf = format!("{}{}", hash1, hash2);

        assert_eq!(merkel_tree(opts), leaf);
    }
}
