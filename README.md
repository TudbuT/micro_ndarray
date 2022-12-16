# micro_ndarray

Most likely the smallest ndarray rust implementation with the best feature/size ratio you will 
find. Did you know `micro_ndarray` is almost exactly 100x smaller than `ndarray` and has no 
dependencies (except for std)?

## Benchmarks

Both implementations are extremely fast considering the number of elements each iteration of 
the benchmark goes through: 10_000_000 relatively equally distributed across the dimensions of the
arrays. However, this changes in 7D, as ndarray only has a man-made fast implementation for up to
7D, while micro_ndarray relies on a single implementation for all dimensions. This makes it slower
in smaller dimensions, but much, much faster in higher dimensions. micro_ndarray only has few 
dimension-dependent optimizations and only for 1D, 2D, and 3D.

In short:
```
micro_ndarry            time:   [24.460 ms 24.602 ms 24.777 ms]
ndarry                  time:   [6.2882 ms 6.3008 ms 6.3148 ms]
micro_ndarry 3D         time:   [43.007 ms 43.054 ms 43.101 ms]
ndarry 3D               time:   [25.750 ms 25.778 ms 25.806 ms]
micro_ndarry 4D         time:   [47.875 ms 47.958 ms 48.043 ms]
ndarry 4D               time:   [39.701 ms 39.756 ms 39.810 ms]
micro_ndarry 7D         time:   [46.657 ms 46.724 ms 46.790 ms]
ndarry 7D               time:   [1.6210 s 1.6228 s 1.6247 s]    <- Boom
```
In order by speed:
```
ndarry                  time:   [6.2882 ms 6.3008 ms 6.3148 ms]
...
micro_ndarry            time:   [24.460 ms 24.602 ms 24.777 ms]
ndarry 3D               time:   [25.750 ms 25.778 ms 25.806 ms]
ndarry 4D               time:   [39.701 ms 39.756 ms 39.810 ms]
micro_ndarry 3D         time:   [43.007 ms 43.054 ms 43.101 ms]
micro_ndarry 7D         time:   [46.657 ms 46.724 ms 46.790 ms] <-\ the difference here is most likely noise
micro_ndarry 4D         time:   [47.875 ms 47.958 ms 48.043 ms] <-/¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯
...
...
[some more ...]
...
...
ndarry 7D               time:   [1.6210 s 1.6228 s 1.6247 s]    <- Boom
```

In long:
```
     Running benches/benchmarks.rs (target/release/deps/benchmarks-37dfbc027b120770)
micro_ndarry            time:   [24.460 ms 24.602 ms 24.777 ms]
                        change: [+0.8630% +1.4891% +2.2669%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe

ndarry                  time:   [6.2882 ms 6.3008 ms 6.3148 ms]
                        change: [-0.7727% -0.5346% -0.2994%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

micro_ndarry 3D         time:   [43.007 ms 43.054 ms 43.101 ms]
                        change: [-1.1832% -1.0392% -0.8876%] (p = 0.00 < 0.05)
                        Change within noise threshold.

ndarry 3D               time:   [25.750 ms 25.778 ms 25.806 ms]
                        change: [-0.6465% -0.5203% -0.3904%] (p = 0.00 < 0.05)
                        Change within noise threshold.

micro_ndarry 4D         time:   [47.875 ms 47.958 ms 48.043 ms]
                        change: [-0.3193% -0.1150% +0.0840%] (p = 0.27 > 0.05)
                        No change in performance detected.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

ndarry 4D               time:   [39.701 ms 39.756 ms 39.810 ms]
                        change: [-0.7251% -0.5708% -0.4248%] (p = 0.00 < 0.05)
                        Change within noise threshold.

micro_ndarry 7D         time:   [46.657 ms 46.724 ms 46.790 ms]
                        change: [-0.3268% -0.1459% +0.0485%] (p = 0.14 > 0.05)
                        No change in performance detected.

Benchmarking ndarry 7D: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 170.6s, or reduce sample count to 10.
ndarry 7D               time:   [1.6210 s 1.6228 s 1.6247 s]
                        change: [+0.4095% +1.6176% +2.8514%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
```

As you can see, micro_ndarray is very consistent even with higher dimensions, while ndarray is 
extremely fast in 2D and exteremely slow in 7D.
