---
sidebar_position: 8
---

# Performance

Last measured on V0.13.0

Measured on an 8 core 2021 MacBook Pro 14-inch, Apple M1 Pro using [criterion](https://github.com/bheisler/criterion.rs):

- 8 lines:

  ```
  compiler/normal (8 lines)                                                                               
                          time:   [4.0464 us 4.0602 us 4.0737 us]
  slope  [4.0464 us 4.0737 us] R^2            [0.9994560 0.9994685]
  mean   [4.0494 us 4.0686 us] std. dev.      [7.8693 ns 19.124 ns]
  median [4.0439 us 4.0788 us] med. abs. dev. [1.7785 ns 25.474 ns]
  ```

- 1M lines:

  ```
  compiler/long input (1M lines)                                                                          
                          time:   [379.99 ms 380.86 ms 381.88 ms]
  mean   [379.99 ms 381.88 ms] std. dev.      [706.66 us 2.2221 ms]
  median [379.37 ms 381.46 ms] med. abs. dev. [54.347 us 2.3614 ms]
  ```

- Deeply nested:

  ```
  compiler/deeply nested  
                          time:   [4.8043 us 4.8093 us 4.8129 us]                                      
  slope  [4.8043 us 4.8129 us] R^2            [0.9999224 0.9999331]
  mean   [4.7961 us 4.8165 us] std. dev.      [5.1109 ns 23.242 ns]
  median [4.7950 us 4.8147 us] med. abs. dev. [1.1498 ns 30.958 ns]
  ```

To reproduce, run `cargo benchmark`