# simd-pcg

This repository is a toy project that exec [PCG](https://www.pcg-random.org/index.html) with SIMD(avx2) in Rust.

I implemented this with reference to [simdpcg](https://github.com/lemire/simdpcg) and [simdxorshift](https://github.com/lemire/SIMDxorshift). Thank you very much.

## benchmark

```bash
rustup run nightly cargo bench
```

### Result

| Loop unrolling Number of instructions | Baseline([rand_pcg](https://crates.io/crates/rand_pcg))  | This implementation |
|-----------------------------|-----------|------------------|
| 1                           | 27.601 ms | 24.447 ms        |
| 2                           | 23.357 ms | 19.803 ms        |
| 4                           | 22.839 ms | 19.526 ms        |
