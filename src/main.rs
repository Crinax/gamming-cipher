mod random;
mod math;

use rand::RngCore;
use random::BbsRand;
use crypto_bigint::{U2048, Random, Uint, Limb};

fn main() {
    let mut bbs = BbsRand::new(11, 19, 3);
    let message = "Привет, привет привет";
    let mut key = vec![0; message.len()];

    bbs.fill_bytes(&mut key);

    let mut ciphered = vec![0; message.len()];
    let mut unciphered = vec![0; message.len()];

    for i in 0..message.len() {
        ciphered[i] = message.as_bytes()[i] ^ key[i];
        unciphered[i] = ciphered[i] ^ key[i];
    }

    println!("Message: {:?}", message.as_bytes());
    println!("Key: {key:?}");
    println!("Ciphered: {ciphered:?}");
    println!("Unciphered: {unciphered:?}");
    println!("Is equals: {}", message.as_bytes() == unciphered);
}
