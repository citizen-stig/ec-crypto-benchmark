RUST=cd rust &&
GO=cd golang &&

init-rust:
	$(RUST)	cargo install

init-go:
	$(GO) go get

init: init-rust init-go

bench-rust:
	$(RUST) cargo bench

bench-go:
	$(GO) go test -bench=.

bench: bench-rust bench-go
