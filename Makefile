.PHONY: all

all: testfile
	cargo build --release
	hyperfine ./target/release/hash_perf
	hyperfine ./hash.py

testfile:
	dd if=/dev/urandom of=testfile bs=1000 count=229000
