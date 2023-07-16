# Performance

Last measured on v0.19.0

Measured on an 8 core 2021 MacBook Pro 14-inch, Apple M1 Pro using [criterion](https://github.com/bheisler/criterion.rs):

- 8 lines:

  ```
  compiler/normal (8 lines)
                            time:   [4.0579 µs 4.0665 µs 4.0788 µs]
  slope  [4.0579 µs 4.0788 µs] R^2            [0.9996538 0.9995633]
  mean   [4.0555 µs 4.0806 µs] std. dev.      [11.018 ns 26.342 ns]
  median [4.0500 µs 4.0852 µs] med. abs. dev. [5.6889 ns 35.806 ns]
  ```

- 1M lines:

  ```
  compiler/long input (1M lines)
                            time:   [400.97 ms 402.31 ms 403.53 ms]
  mean   [400.97 ms 403.53 ms] std. dev.      [773.42 µs 2.9886 ms]
  median [401.22 ms 403.39 ms] med. abs. dev. [59.042 µs 3.5129 ms]
  ```

- Deeply nested:

  ```
  compiler/deeply nested
                            time:   [5.8085 µs 5.8291 µs 5.8514 µs]
  slope  [5.8085 µs 5.8514 µs] R^2            [0.9992861 0.9992461]
  mean   [5.8064 µs 5.8519 µs] std. dev.      [21.027 ns 49.152 ns]
  median [5.7949 µs 5.8583 µs] med. abs. dev. [3.3348 ns 64.628 ns]
  ```

To reproduce, run `cargo bench` or `cargo xtask benchmark`