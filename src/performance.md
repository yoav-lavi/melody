# Performance

Last measured on v0.20.0

Measured on an 8 core 2021 MacBook Pro 14-inch, Apple M1 Pro using [criterion](https://github.com/bheisler/criterion.rs):

- 8 lines:

  ```
  compiler/normal (8 lines)
                            time:   [4.3556 µs 4.3674 µs 4.3751 µs]
  slope  [4.3556 µs 4.3751 µs] R^2            [0.9996144 0.9996931]
  mean   [4.3377 µs 4.3678 µs] std. dev.      [16.019 ns 30.154 ns]
  median [4.3270 µs 4.3777 µs] med. abs. dev. [3.1402 ns 41.334 ns]
  ```

- 1M lines:

  ```
  compiler/long input (1M lines)
                            time:   [470.04 ms 472.35 ms 474.78 ms]
  mean   [470.04 ms 474.78 ms] std. dev.      [2.0458 ms 5.3453 ms]
  median [469.54 ms 475.24 ms] med. abs. dev. [734.10 µs 6.8144 ms]
  ```

- Deeply nested:

  ```
  compiler/deeply nested
                            time:   [4.2357 µs 4.2561 µs 4.2782 µs]
  slope  [4.2357 µs 4.2782 µs] R^2            [0.9988854 0.9988087]
  mean   [4.2474 µs 4.2752 µs] std. dev.      [13.698 ns 29.574 ns]
  median [4.2426 µs 4.2819 µs] med. abs. dev. [2.7127 ns 43.193 ns]
  ```

To reproduce, run `cargo bench` or `cargo xtask benchmark`
