use lambdaworks_math::{
    field::fields::u64_prime_field::{U64FieldElement, FE17},
    polynomial::Polynomial,
};
use rand::Rng;

const MODULUS: u64 = 17;
type FE = U64FieldElement<MODULUS>;

// Convert a string to a decimal number
fn string_to_decimal(secret: &str) -> FE {
    let mut secret_binary = String::new();
    for c in secret.chars() {
        let binary = format!("{:08b}", c as u8);
        secret_binary.push_str(&binary);
    }
    println!("Secret binary: {:?}", secret_binary);
    let secret_int = u64::from_str_radix(&secret_binary, 2).unwrap();
    FE::new(secret_int)
}

fn construct_polynomial(secret: FE, k: FE) -> Polynomial<FE17> {
    let mut coefficients = Vec::new();
    coefficients.push(secret);
    // k is the threshold, polynomial degree is k - 1
    for _ in 0..(k.representative() - 1) {
        // select random coefficients
        let coeffient = rand::thread_rng().gen_range(1..100) as u64;
        coefficients.push(FE::new(coeffient));
    }
    Polynomial::new(&coefficients)
}

#[derive(Debug, Clone, Copy)]
struct Share {
    x: FE,
    y: FE,
}

fn find_shares(polynomial: Polynomial<FE17>, n: u64) -> Vec<Share> {
    let mut shares = Vec::new();
    for i in 1..=n {
        let share = polynomial.evaluate(&FE::new(i));
        shares.push(Share {
            x: FE::new(i),
            y: share,
        });
    }
    shares
}

fn select_random_shares(shares: Vec<Share>, k: FE) -> Vec<Share> {
    let mut selected_shares = Vec::new();
    for _ in 0..k.representative() {
        let share_idx = rand::thread_rng().gen_range(0..shares.len()) as usize;
        selected_shares.push(shares[share_idx]);
    }
    selected_shares
}

fn main() {
    // cannot be too longer than 64 bits
    let secret = string_to_decimal("hello");
    println!("Secret: {:?}", secret);

    // decide threshold
    let k: FE = FE::new(3);
    let polynomial = construct_polynomial(secret, k);
    println!("Polynomial: {:?}", polynomial);

    let n = 5;
    let shares = find_shares(polynomial, n);
    println!("Shares: {:#?}", shares);

    // reconstruct the secret
    // pick random k shares outof n shares
    let selected_shares = select_random_shares(shares, k);
    println!("Selected shares: {:#?}", selected_shares);

    // determine the secret
    let xs = selected_shares.iter().map(|s| s.x).collect::<Vec<FE>>();
    let ys = selected_shares.iter().map(|s| s.y).collect::<Vec<FE>>();
    let interpolated_polynomial = Polynomial::interpolate(&xs, &ys).unwrap();

    println!("Interpolated polynomial: {:?}", interpolated_polynomial);
    let secret_reconstructed = interpolated_polynomial.evaluate(&FE::new(0));
    assert_eq!(secret, secret_reconstructed);
}
