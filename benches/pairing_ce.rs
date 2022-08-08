use criterion::{black_box, criterion_group, criterion_main, Criterion};

use pairing_ce::bn256::*;
use pairing_ce::{CurveProjective};
use rand::{Rand, SeedableRng, XorShiftRng};

fn mul_pairing_ce(c: &mut Criterion) {
    let g1 = G1::one();
    println!("pairing_ce G1: {:?}", g1);
    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);
    let fr = Fr::rand(&mut rng);
    println!("FR: {:?}", fr);
    // "0x0000000000000000000000000000000000000000000000000000000000000001"
    // "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
    // let fr: _ = from_hex::<Fr>("0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap();
    // g1.mul_assign(fr);
    // println!("G1: {:?}", g1);
    c.bench_function("pairing_ce.mul_assign", |b| b.iter(|| {
        let mut local_g1 = g1.clone();
        local_g1.mul_assign(black_box(fr));
    }));
}

criterion_group!(benches, mul_pairing_ce);
criterion_main!(benches);
