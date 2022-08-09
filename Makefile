RUST=cd rust &&
GO=cd golang &&
CPP=cd cpp &&

# Inits
init-rust:
	$(RUST)	cargo build

init-go:
	$(GO) go get

init-cpp:
	$(CPP) sh ./deps.sh
	$(CPP) make all

init: init-rust init-go

# Benches
bench-rust:
	$(RUST) cargo bench

bench-go:
	$(GO) go test -bench=.

bench-cpp:
	$(CPP) ./mcl_bench

bench: bench-rust bench-go bench-cpp


# Cleans
clean-rust:
	$(RUST) cargo clean

clean-go:
	$(GO) go clean -modcache

clean-cpp:
	$(CPP) clean
	$(CPP) rm -rf mcl
	$(CPP) rm -rf benchmark

clean: clean-rust clean-go clean-cpp
