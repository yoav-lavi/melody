# Performance

Last measured on V0.13.3

Measured on an 8 core 2021 MacBook Pro 14-inch, Apple M1 Pro using [criterion](https://github.com/bheisler/criterion.rs):

- 8 lines:

  ```
  compiler/normal (8 lines)
                          time:   [3.6560 us 3.6596 us 3.6644 us]
  slope  [3.6560 us 3.6644 us] R^2            [0.9999367 0.9999233]
  mean   [3.6577 us 3.6676 us] std. dev.      [3.2234 ns 11.399 ns]
  median [3.6549 us 3.6674 us] med. abs. dev. [642.70 ps 12.973 ns]
  ```

- 1M lines:

  ```
  compiler/long input (1M lines)
                          time:   [345.99 ms 348.85 ms 351.91 ms]
  mean   [345.99 ms 351.91 ms] std. dev.      [2.8317 ms 6.3397 ms]
  median [344.55 ms 352.85 ms] med. abs. dev. [893.36 us 8.5853 ms]
  ```

- Deeply nested:

  ```
  compiler/deeply nested
                          time:   [4.8259 us 4.8330 us 4.8399 us]
  slope  [4.8259 us 4.8399 us] R^2            [0.9998793 0.9998830]
  mean   [4.8259 us 4.8476 us] std. dev.      [7.6412 ns 24.306 ns]
  median [4.8234 us 4.8484 us] med. abs. dev. [4.1349 ns 30.340 ns]
  ```

To reproduce, run `cargo benchmark`