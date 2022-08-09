package golang

import (
	"github.com/consensys/gnark-crypto/ecc/bn254"
	"math/big"
	"testing"
)

func BenchmarkGnarkCryptoMulAffine(b *testing.B) {
	var g bn254.G1Affine
	g.X.SetInt64(1)
	g.Y.SetInt64(2)

	var s big.Int
	s.SetString("30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000000", 16)

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		var res bn254.G1Affine
		res.ScalarMultiplication(&g, &s)
	}
}

func BenchmarkGnarkCryptoMulJac(b *testing.B) {
	var g bn254.G1Jac
	g.X.SetInt64(1)
	g.Y.SetInt64(2)
	g.Z.SetInt64(1)

	var s big.Int
	s.SetString("30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000000", 16)

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		var res bn254.G1Jac
		res.ScalarMultiplication(&g, &s)
	}
}
