---
sidebar_position: 8
---

# Performance

Last measured on V0.12.4

Measured on an 8 core 2021 MacBook Pro 14-inch, Apple M1 Pro using [criterion](https://github.com/bheisler/criterion.rs):

- 8 lines:

  ```
  compiler/normal (8 lines)                                                                               
                          time:   [4.1423 us 4.1555 us 4.1699 us]
  slope  [4.1423 us 4.1699 us] R^2            [0.9994903 0.9994576]
  mean   [4.1412 us 4.1608 us] std. dev.      [7.9841 ns 21.816 ns]
  median [4.1370 us 4.1633 us] med. abs. dev. [3.2990 ns 29.024 ns]
  ```

- 1M lines:

  ```
  compiler/long input (1M lines)                                                                          
                          time:   [386.65 ms 388.91 ms 391.28 ms]
  mean   [386.65 ms 391.28 ms] std. dev.      [2.2683 ms 4.7309 ms]
  median [384.62 ms 393.30 ms] med. abs. dev. [158.47 us 6.5765 ms]
  ```

- Deeply nested:

  ```
  compiler/deeply nested  
                          time:   [4.7804 us 4.7903 us 4.8036 us]                                      
  slope  [4.7804 us 4.8036 us] R^2            [0.9997212 0.9996534]
  mean   [4.7856 us 4.8075 us] std. dev.      [9.2709 ns 23.911 ns]
  median [4.7801 us 4.8113 us] med. abs. dev. [3.5263 ns 30.781 ns]
  ```

To reproduce, run `cargo benchmark`