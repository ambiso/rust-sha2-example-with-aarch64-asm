Expected output:

```sh
$ make
cargo build --release
    Finished release [optimized] target(s) in 0.00s
hyperfine ./target/release/hash_perf
Benchmark 1: ./target/release/hash_perf
  Time (mean ± σ):     112.6 ms ±   2.7 ms    [User: 95.4 ms, System: 17.0 ms]
  Range (min … max):   109.2 ms … 118.5 ms    25 runs
 
hyperfine ./hash.py
Benchmark 1: ./hash.py
  Time (mean ± σ):     138.0 ms ±   5.5 ms    [User: 109.2 ms, System: 28.6 ms]
  Range (min … max):   132.1 ms … 154.1 ms    21 runs
 
```