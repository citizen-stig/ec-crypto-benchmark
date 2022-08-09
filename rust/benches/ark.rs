use criterion::{black_box, criterion_group, criterion_main, Criterion};

use ark_ff::fields::PrimeField;
use ark_ec::AffineCurve;
use ark_bn254::{G1Affine, Fr, Fq};
use ark_ff::Zero;
use num_bigint::BigUint;


fn mul_ark(c: &mut Criterion) {
    let x = Fq::from(1);
    let y = Fq::from(2);
    let g1 = G1Affine::new(x, y, false);
    let max_element = "21888242871839275222246405745257275088548364400416034343698204186575808495616".parse::<BigUint>().unwrap();
    let s = Fr::from_be_bytes_mod_order(&max_element.to_bytes_be());

    c.bench_function("ark-bn254.mul", |b| b.iter(|| {
        let res = g1.mul(black_box(s));
        assert!(!res.is_zero())
    }));
}

criterion_group!(benches, mul_ark);
criterion_main!(benches);
