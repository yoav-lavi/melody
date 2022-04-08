# Performance

Last measured on v0.13.10

Measured on an 8 core 2021 MacBook Pro 14-inch, Apple M1 Pro using [criterion](https://github.com/bheisler/criterion.rs):

- 8 lines:

  ```
  compiler/normal (8 lines)                        
                          time:   [3.6734 us 3.6775 us 3.6809 us]
  slope  [3.6734 us 3.6809 us] R^2            [0.9999393 0.9999460]
  mean   [3.6726 us 3.6854 us] std. dev.      [3.8234 ns 15.619 ns]
  median [3.6703 us 3.6833 us] med. abs. dev. [1.3873 ns 14.729 ns]
  ```

- 1M lines:

  ```
  compiler/long input (1M lines)                        
                          time:   [344.68 ms 346.83 ms 349.29 ms]
  mean   [344.68 ms 349.29 ms] std. dev.      [1.4962 ms 4.9835 ms]
  median [344.16 ms 350.06 ms] med. abs. dev. [407.85 us 6.3428 ms]
  ```

- Deeply nested:

  ```
  compiler/deeply nested  
                          time:   [3.8017 us 3.8150 us 3.8342 us]
  slope  [3.8017 us 3.8342 us] R^2            [0.9992078 0.9989523]
  mean   [3.8158 us 3.8656 us] std. dev.      [8.8095 ns 65.691 ns]
  median [3.8144 us 3.8397 us] med. abs. dev. [2.5630 ns 40.223 ns]
  ```

To reproduce, run `cargo bench` or `cargo xtask benchmark`