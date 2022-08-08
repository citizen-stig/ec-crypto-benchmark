use criterion::{black_box, criterion_group, criterion_main, Criterion};

use eth_pairings::engines::bn254::*;
use eth_pairings::public_interface::decode_g1::serialize_g1_point;
use eth_pairings::public_interface::eip196::EIP196Executor;
use eth_pairings::weierstrass::Group;

fn mul_eth_pairings(c: &mut Criterion) {
    // let p_x = vec![212, 90, 205, 88, 47, 240, 131, 117, 58, 234, 55, 169, 156, 27, 91, 70, 205, 209, 135];
    // let p_y = vec![100, 20, 30];
    // let p_x = Fp::from_be_bytes(&*BN254_BASE_FIELD, &p_x, true).unwrap();
    // let p_y = Fp::from_be_bytes(&*BN254_BASE_FIELD, &p_y, true).unwrap();
    // let point = CurvePoint::point_from_xy(&*BN254_G1_CURVE, p_x, p_y);
    let g1 = &*BN254_G1_GENERATOR;
    println!("eth_pairings g1: {:?}", g1.into_xy());
    let mut serialized = serialize_g1_point(32, g1).unwrap();
    let worst_case_scalar = [u64::MAX; 4];
    println!("WORST SCALAR: {:?}", &worst_case_scalar);

    for i in worst_case_scalar {
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
