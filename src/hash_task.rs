extern crate secp256k1;

use std::cmp;
use std::time::{SystemTime};
use core::fmt::Write;

use secp256k1::rand::rngs::OsRng;
use secp256k1::{Secp256k1};
use sha3::{Digest, Keccak256};


pub fn run (max_hash: usize) -> usize {
    let now = SystemTime::now();
    let mut counter = 0;
    let secp = Secp256k1::new();
    let mut rng: OsRng;
    loop {
        rng = OsRng::new().unwrap();
        let (private_key, public_key) = secp.generate_keypair(&mut rng);
        let public_key = &public_key.serialize_uncompressed();
        let addr_s = checksum_address(public_key);
        counter += 1;
        if counter == max_hash {
            break;
        }

        let max_repeat = check_max_repeat(&addr_s);
        if max_repeat > 10 {
            println!("no: {} repeated: {}", counter, max_repeat);
            println!("addr: {} \nsecret: {} \npub: {}\n", addr_s, private_key, to_hex_string(public_key));
        }
    }
    let end = now.elapsed().expect("elapsed failed");
    let perf = max_hash as f32 / (end.as_millis() / 1000) as f32;
    println!("generated {} in {}", max_hash, end.as_secs());
    println!("the performance is : {}", perf);
    perf as usize
}

pub fn check_max_repeat(address: &String) -> i8 {
    let mut max = 1;
    let mut last = 'x';
    let mut cur = 1;
    let shorted = &address[2..];

    for c in shorted.chars() {
        if c == last {
            cur += 1;
            max = cmp::max(cur, max);
            continue
        }

        last = c;
        cur = 1;
    }
    max
}

pub fn to_hex_string(pubkey: &[u8]) -> String {
    let mut pubkey_s = String::with_capacity(2 * pubkey.len());
    for byte in pubkey {
        write!(pubkey_s, "{:02X}", byte).expect("hex write failed");
    }
    pubkey_s
}

pub fn checksum_address(pubhash: &[u8]) -> String {
    let mut hasher = Keccak256::new();
    let sliced = &pubhash[1..];
    hasher.update(sliced);
    let hash = hasher.finalize().clone();
    let address = to_hex_string(&hash[12..]).to_lowercase();
    let mut hasher = Keccak256::new();
    hasher.update(address.as_bytes());
    let hash = to_hex_string(&hasher.finalize());
    let mut checksum_address = "0x".to_string();
    for c in 0..40 {
        let ch = match &hash[c..=c] {
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" => address[c..=c].to_lowercase(),
            _ => address[c..=c].to_uppercase(),
        };
        checksum_address.push_str(&ch);
    }

    checksum_address
}