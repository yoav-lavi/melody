---
sidebar_position: 8
---

# Performance

Last measured on v0.12.0

Measured on a 8 core 2021 MacBook Pro 14-inch, Apple M1 Pro using [criterion](https://github.com/bheisler/criterion.rs):

- 8 lines:
  ```
  compiler/normal (8 lines)                                                                               
                          time:   [4.4636 us 4.4911 us 4.5162 us]
  slope  [4.4636 us 4.5162 us] R^2            [0.9978146 0.9979425]
  mean   [4.4456 us 4.5138 us] std. dev.      [30.807 ns 78.087 ns]
  median [4.4405 us 4.5161 us] med. abs. dev. [1.7263 ns 102.81 ns]
  ```

- 1M lines:
  ```
  compiler/long input (1M lines)                                                                          
                          time:   [414.41 ms 418.28 ms 421.92 ms]
  mean   [414.41 ms 421.92 ms] std. dev.      [3.9838 ms 7.6492 ms]
  median [412.74 ms 424.18 ms] med. abs. dev. [613.87 us 11.100 ms]
  ```

- Deeply nested:

  ```
  compiler/deeply nested  
                          time:   [5.0808 us 5.1471 us 5.2011 us]
  slope  [5.0808 us 5.2011 us] R^2            [0.9926848 0.9936608]
  mean   [5.0739 us 5.1884 us] std. dev.      [64.329 ns 110.75 ns]
  median [5.0313 us 5.2249 us] med. abs. dev. [6.1076 ns 151.57 ns]
  ```

To reproduce, run `cargo benchmark`