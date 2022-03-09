---
sidebar_position: 8
---

# Performance

Last measured on V0.13.2

Measured on an 8 core 2021 MacBook Pro 14-inch, Apple M1 Pro using [criterion](https://github.com/bheisler/criterion.rs):

- 8 lines:

  ```
  compiler/normal (8 lines)
                          time:   [3.6689 us 3.6741 us 3.6781 us]
  slope  [3.6689 us 3.6781 us] R^2            [0.9999147 0.9999302]
  mean   [3.6705 us 3.6798 us] std. dev.      [4.8109 ns 9.6190 ns]
  median [3.6683 us 3.6820 us] med. abs. dev. [1.4209 ns 13.508 ns]
  ```

- 1M lines:

  ```
  compiler/long input (1M lines)
                          time:   [3.6689 us 3.6741 us 3.6781 us]
  mean   [346.12 ms 350.70 ms] std. dev.      [659.46 us 5.0580 ms]
  median [345.91 ms 350.70 ms] med. abs. dev. [176.21 us 5.3785 ms]
  ```

- Deeply nested:

  ```
  compiler/deeply nested
                          time:   [4.8100 us 4.8228 us 4.8336 us]
  slope  [4.8100 us 4.8336 us] R^2            [0.9997070 0.9997448]
  mean   [4.8162 us 4.8298 us] std. dev.      [4.9737 ns 16.288 ns]
  median [4.8181 us 4.8311 us] med. abs. dev. [885.24 ps 16.563 ns]
  ```