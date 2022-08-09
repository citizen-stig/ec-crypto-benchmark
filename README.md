# Description

Contains benchmarks of elliptic curve multiplication on BN254 (aka BN256, alt-bn128) using these libraries:

 * Rust: [EIP-1962](https://github.com/matter-labs/eip1962)
 * Rust: [Pairing CE](https://github.com/ConsenSys/pairing)
 * Rust: [Ark-BN254](https://github.com/arkworks-rs/curves/tree/master/bn254)
 * Golang: [Gnark crypto](https://github.com/ConsenSys/gnark-crypto)
 * CPP: [MCL](https://github.com/herumi/mcl/)


# Run

Requirements:
 * rustup: cargo and rustc
 * golang
 * cmake and cpp compiler (gcc or clang)
 * git

```
make init
make bench
```
