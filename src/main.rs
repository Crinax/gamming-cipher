use crypto_bigint::Uint;

fn main() {
    println!("{}", Uint::<1>::new([crypto_bigint::Limb::from(0u8)]) ^ Uint::<1>::from([crypto_bigint::Limb::from(1u8)]));
}
