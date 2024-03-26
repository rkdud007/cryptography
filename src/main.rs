use lambdaworks_math::{
    cyclic_group::IsGroup,
    elliptic_curve::{
        short_weierstrass::curves::bls12_381::curve::BLS12381Curve, traits::IsEllipticCurve,
    },
};

fn main() {
    let priv_key: u128 = 0x6C616D6264617370;
    let g = BLS12381Curve::generator();
    let g2 = g.operate_with_self(priv_key);
    let g2_affine = g2.to_affine();
    let x = g2_affine.x();
    let y = g2_affine.y();

    println!(
        "public key: {:?}, it is point (x,y) = ({:?},{:?})",
        g2,
        x.to_hex(),
        y.to_hex()
    );
}
