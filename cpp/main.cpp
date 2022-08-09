#include <iostream>
// BN254 ; sizeof(Fp) = 32, sizeof(Fr) = 32
#define MCL_MAX_FP_BIT_SIZE 256

#include <mcl/bn.hpp>
#include <benchmark/benchmark.h>

#define PUT(x) std::cout << #x "=" << (x) << std::endl;

static void MCL_BM_BN254_Mul(benchmark::State &state) {
    using namespace mcl;
//    printf("Starting setup\n");

    mpz_class s;
    s.setStr("21888242871839275222246405745257275088548364400416034343698204186575808495616");

    bn::initPairing(mcl::BN254);
    bn::G1 P1;
    bn::G1 P2;
    P1.x.setStr("1");
    P1.y.setStr("2");
    P1.z.setStr("1");

//    PUT(P1);

    // Fails
//    assert(P1.isValid());
    assert(!P1.isZero());

//    PUT(P1);

//    printf("DONE SETUP \n");
    for (auto _: state) {
        P2 = P1 * s;
        assert(!P2.isZero());
    }
}

// Register the function as a benchmark
BENCHMARK(MCL_BM_BN254_Mul);

BENCHMARK_MAIN();

//int main()
//try
//{
//    using namespace mcl;
//    printf("Hello world\n");
//
//    mpz_class s;
//    s.setStr("21888242871839275222246405745257275088548364400416034343698204186575808495616");
//
//    bn::initPairing(mcl::BN254);
//    bn::G1 P1;
//    P1.x.setStr("1");
//    P1.y.setStr("2");
//    P1.z.setStr("3");
//
//    PUT(P1);
//
//    P1 *= s;
//
//    PUT(P1);
//
//    printf("DONE!\n");
//
//} catch (std::exception& e) {
//    printf("err %s\n", e.what());
//    return 1;
//}
