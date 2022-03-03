---
sidebar_position: 8
---

# Performance

Last measured on v0.11.0

Measured on a 8 core 2021 MacBook Pro 14-inch, Apple M1 Pro using [hyperfine](https://github.com/sharkdp/hyperfine), compiling a 2.3M LOC file via the Melody CLI (the file was made using a repetition of the examples in this repository):

```
Time (mean ± σ):      1.002 s ±  0.010 s    [User: 0.905 s, System: 0.093 s]
Range (min … max):    0.984 s …  1.016 s    20 runs
```

For real world usage (on similar hardware), expect less than 1 ms (0.8 ms on a 100 LOC file, but hyperfine might be inaccurate under 5 ms)
