use criterion::{black_box, criterion_group, criterion_main, Criterion};

use pairing_ce::bn256::*;
use pairing_ce::{CurveAffine, CurveProjective, from_hex};

fn mul_pairing_ce(c: &mut Criterion) {
    let g1 = G1::one().into_affine();
    println!("pairing_ce G1: {:?}", g1);
    let fr: Fr = from_hex::<Fr>("0x30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000000").unwrap();
    c.bench_function("pairing_ce.mul_assign", |b| b.iter(|| {
        let res = g1.mul(black_box(fr));
        assert!(!res.is_zero())
    }));
}

criterion_group!(benches, mul_pairing_ce);
criterion_main!(benches);
