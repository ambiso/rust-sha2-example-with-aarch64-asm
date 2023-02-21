Expected output:

```sh
$ make
cargo build --release
    Finished release [optimized] target(s) in 0.00s
hyperfine ./target/release/hash_perf
Benchmark 1: ./target/release/hash_perf
  Time (mean ± σ):     123.0 ms ±   3.0 ms    [User: 97.4 ms, System: 25.5 ms]
  Range (min … max):   119.1 ms … 128.3 ms    24 runs
 
hyperfine ./hash.py
Benchmark 1: ./hash.py
  Time (mean ± σ):     141.8 ms ±   4.6 ms    [User: 110.8 ms, System: 30.6 ms]
  Range (min … max):   133.4 ms … 152.8 ms    20 runs
```