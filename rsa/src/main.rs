use lambdaworks_math::field::{fields::u64_goldilocks_field::Goldilocks64Field, traits::IsField};
use prime_factorization::Factorization;
use rand::Rng;

type F = Goldilocks64Field;

pub struct Key {
    n: u64,
    e: u64,
    d: u64,
}

pub struct PublicKey {
    n: u64,
    e: u64,
}

impl PublicKey {
    pub fn new(n: u64, e: u64) -> Self {
        Self { n, e }
    }

    pub fn encrypt(&self, m: u64) -> u64 {
        modpow(m, self.e, self.n)
    }
}

impl Key {
    pub fn generate() -> Self {
        let prime_index_1 = rand::thread_rng().gen_range(1000..1100) as usize;
        let prime_index_2 = rand::thread_rng().gen_range(1000..1100) as usize;
        let prime_1 = primal::Primes::all().nth(prime_index_1).unwrap() as u64;
        let prime_2 = primal::Primes::all().nth(prime_index_2).unwrap() as u64;
        // pick two large prime number p, q (secret)
        let p = F::from_u64(prime_1);
        let q = F::from_u64(prime_2);
        let n = F::mul(&p, &q);
        // Euler's totient function (z = φ(n))
        let z = F::mul(&F::sub(&p, &F::one()), &F::sub(&q, &F::one()));
        let e = search_e(z);
        // (e * d) mod n = 1
        let d = search_d(e, z);
        Self { n, e, d }
    }

    pub fn public_key(&self) -> PublicKey {
        PublicKey::new(self.n, self.e)
    }

    pub fn private_key(&self) -> (u64, u64) {
        (self.n, self.d)
    }

    pub fn decrypt(&self, c: u64) -> u64 {
        modpow(c, self.d, self.n)
    }
}

// calculate (base^exp) % modulus efficiently
fn modpow(base: u64, exp: u64, modulus: u64) -> u64 {
    let mut result = 1;
    let mut base = base % modulus;
    let mut exp = exp;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        base = (base * base) % modulus;
        exp /= 2;
    }

    result
}

// search e where 1 < e < z and gcd(e, z) = 1
fn search_e(z: u64) -> u64 {
    let factor_repr = Factorization::run(z);
    loop {
        let e = rand::thread_rng().gen_range(2..z);
        let factor_repr_e = Factorization::run(e);
        // gcd(e, z) = 1
        if factor_repr
            .factors
            .iter()
            .all(|f| !factor_repr_e.factors.contains(f))
        {
            return e;
        }
    }
}

// search d where (e * d) mod z = 1
fn search_d(e: u64, z: u64) -> u64 {
    let mut d = 1;
    loop {
        if F::mul(&e, &d) % z == 1 {
            return d;
        }
        d += 1;
    }
}

fn main() {
    // 1. pick two large prime number p, q (secret)
    let key = Key::generate();

    // --- encryption ---
    // 2. public key (n,e)
    let public_key = key.public_key();
    let original_m = F::from_u64(10000); // message
    let c = public_key.encrypt(original_m); // c = m^e mod n

    // --- decryption ---
    // 3. private key (n,d)
    let decrypted_m = key.decrypt(c); // m = c^d mod n
    assert_eq!(decrypted_m, original_m);
    println!("decrypted message: {}", decrypted_m);
}
