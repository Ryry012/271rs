use num_bigint::BigInt;
use num_traits::{Zero, One, Euclid};
use num_integer::Integer;
use sha2::{Digest, Sha512};

pub fn h(m: &[u8]) -> Vec<u8> {
    let mut hasher = Sha512::new();
    hasher.update(m);
    hasher.finalize().to_vec()
}

pub fn bit(h_val: &[u8], i: usize) -> u8 {
    let byte = h_val[i / 8];
    (byte >> (i % 8)) & 1
}

pub fn expmod(b_val: &BigInt, e: &BigInt, m: &BigInt) -> BigInt {
    if e.is_zero() {
        return BigInt::one();
    }
    let mut result = BigInt::one();
    let mut base = b_val.mod_floor(m);
    let mut exp = e.clone();
    while exp > BigInt::zero() {
        if (&exp & BigInt::one()) == BigInt::one() {
            result = (result * &base).mod_floor(m);
        }
        base = (&base * &base).mod_floor(m);
        exp >>= 1;
    }
    result
}

pub fn inv(x: &BigInt, q: &BigInt) -> BigInt {
    expmod(x, &(q - 2u32), q)
}

pub fn xrecover(y: &BigInt, q: &BigInt, d: &BigInt, i_const: &BigInt) -> BigInt {
    let y2 = (y * y).mod_floor(q);
    let num = (&y2 - BigInt::one()).mod_floor(q);
    let den = (BigInt::one() + d * &y2).mod_floor(q);
    let inv_den = inv(&den, q);
    let x2 = (num * inv_den).mod_floor(q);
    let x = expmod(&x2, &((q + 3u32) / 8u32), q);
    if (&x * &x).mod_floor(q) == x2 {
        x
    } else {
        (x * i_const).mod_floor(q)
    }
}

pub fn edwards(p: &Vec<BigInt>, q_val: &Vec<BigInt>, q: &BigInt, d: &BigInt) -> Vec<BigInt> {
    unimplemented!()
}

pub fn scalarmult(p: &Vec<BigInt>, e: &BigInt, q: &BigInt, d: &BigInt) -> Vec<BigInt> {
    unimplemented!()
}

pub fn encodeint(y: &BigInt, b: usize) -> Vec<u8> {
    unimplemented!()
}

pub fn encodepoint(p: &Vec<BigInt>, b: usize) -> Vec<u8> {
    unimplemented!()
}

pub fn publickey(sk: &[u8], b: usize, q: &BigInt, d: &BigInt, b_point: &Vec<BigInt>) -> Vec<u8> {
    unimplemented!()
}

pub fn hint(m: &[u8], b: usize) -> BigInt {
    unimplemented!()
}

pub fn signature(
    m: u8,
    sk: &[u8],
    pk: &[u8],
    b: usize,
    q: &BigInt,
    l: &BigInt,
    d: &BigInt,
    b_point: &Vec<BigInt>,
) -> Vec<u8> {
    unimplemented!()
}

pub fn isoncurve(p: &Vec<BigInt>, q: &BigInt, d: &BigInt) -> bool {
    unimplemented!()
}

pub fn decodeint(s: &[u8], b: usize) -> BigInt {
    unimplemented!()
}

pub fn decodepoint(
    s: &[u8],
    b: usize,
    q: &BigInt,
    d: &BigInt,
    i_const: &BigInt
) -> Result<Vec<BigInt>, &'static str> {
    unimplemented!()
}

pub fn checkvalid(
    s: &[u8],
    m: u8,
    pk: &[u8],
    b: usize,
    q: &BigInt,
    d: &BigInt,
    i_const: &BigInt,
    b_point: &Vec<BigInt>
) -> bool {
    unimplemented!()
}
