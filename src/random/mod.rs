use std::fmt::Display;

use crate::math::{power_by_mod, find_lcm_from_gcd, extend_gcd, cropping_modulo};

#[derive(Clone, Copy, Debug, Default)]
pub struct BbsRand {
    p: u8,
    q: u8,
    m: i128,
    first: u32,
    current: usize,
}

enum BbsRandError {
    InvalidDest,
    InvalidPQ
}

impl Display for BbsRandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BbsRandError::InvalidDest => f.write_str("Invalid dest: dest length < 255"),
            BbsRandError::InvalidPQ => f.write_str("Invalid p and q: p * q < 255"),
        }
    }
}

impl BbsRand {
    pub fn new(p: u8, q: u8, seed: i128) -> Self {
        let m = (p as i128) * (q as i128);
        Self { p, q, first: cropping_modulo(seed.pow(2), m) as u32, m, current: 2 }
    }
    
    pub fn fill_bytes(&mut self, dest: &mut [u8]) {
        if let Err(e) = self.try_fill_bytes(dest) {
            panic!("Error: {e}");
        }
    }

    pub fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), BbsRandError> {
        if dest.len() > 255 {
            return Err(BbsRandError::InvalidDest);
        }

        if let None = self.p.checked_mul(self.q) {
            return Err(BbsRandError::InvalidPQ);
        }

        let p = (self.p as i128) - 1;
        let q = (self.q as i128) - 1;
        let gcd = extend_gcd(p, q);
        let carmichael_fn_result = find_lcm_from_gcd(gcd.gcd, p, q);

        for i in 0..dest.len() {
            dest[i] = power_by_mod(
                self.first as i128,
                power_by_mod(2, self.current as i128, carmichael_fn_result),
                self.m
            ) as u8;
            self.current += 1;
        }

        Ok(())
    }
}
