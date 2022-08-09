use criterion::{black_box, criterion_group, criterion_main, Criterion};

use eth_pairings::engines::bn254::*;
use eth_pairings::public_interface::decode_g1::serialize_g1_point;
use eth_pairings::public_interface::eip196::EIP196Executor;
use eth_pairings::weierstrass::Group;

use num_bigint::BigUint;

fn mul_eth_pairings(c: &mut Criterion) {
    let g1 = &*BN254_G1_GENERATOR;
    println!("eth_pairings g1: {:?}", g1.into_xy());
    let mut serialized = serialize_g1_point(32, g1).unwrap();
    let max_element = "21888242871839275222246405745257275088548364400416034343698204186575808495616".parse::<BigUint>().unwrap();
    let worst_case_scalar = max_element.to_u64_digits();

    for i in &worst_case_scalar {
        let mut be = i.to_be_bytes().to_vec();
        serialized.append(&mut be);
    }

    c.bench_function("eth_pairings.point.mul", |b| b.iter(|| assert!(!g1.mul(black_box(&worst_case_scalar[..])).is_zero())));
    c.bench_function("eth_pairings.EIP196Executor::mul", |b| b.iter(|| {
        let result = EIP196Executor::mul(black_box(&serialized)).unwrap();
        assert!(result[0] > 0);
    }));
}



criterion_group!(benches, mul_eth_pairings);
criterion_main!(benches);
