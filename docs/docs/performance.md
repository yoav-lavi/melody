---
sidebar_position: 8
---

# Performance

Last measured on V0.12.2

Measured on an 8 core 2021 MacBook Pro 14-inch, Apple M1 Pro using [criterion](https://github.com/bheisler/criterion.rs):

- 8 lines:
  ```
  compiler/normal (8 lines)                                                                               
                          time:   [4.1149 us 4.1229 us 4.1297 us]
  slope  [4.1149 us 4.1297 us] R^2            [0.9998391 0.9998566]
  mean   [4.1158 us 4.1267 us] std. dev.      [5.5789 ns 11.255 ns]
  median [4.1129 us 4.1300 us] med. abs. dev. [530.77 ps 16.729 ns]
  ```

- 1M lines:
  ```
  compiler/long input (1M lines)                                                                          
                          time:   [4.7121 us 4.7268 us 4.7395 us]                                      
  slope  [4.7121 us 4.7395 us] R^2            [0.9995899 0.9996336]
  mean   [4.7087 us 4.7311 us] std. dev.      [10.236 ns 25.070 ns]
  median [4.7055 us 4.7333 us] med. abs. dev. [3.0236 ns 32.995 ns]
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