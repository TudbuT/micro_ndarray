# micro_ndarray

Most likely the smallest ndarray rust implementation with the best feature/size ratio you will 
find.

## Benchmarks

```
     Running benches/benchmarks.rs (target/release/deps/benchmarks-1e1fde2e14bfecef)
micro_ndarry            time:   [24.670 ms 24.819 ms 24.984 ms]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe

ndarry                  time:   [6.5414 ms 6.5628 ms 6.5856 ms]
Found 20 outliers among 100 measurements (20.00%)
  1 (1.00%) high mild
  19 (19.00%) high severe

micro_ndarry 7D         time:   [37.230 ms 37.307 ms 37.386 ms]

Benchmarking ndarry 7D: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 75.9s, or reduce sample count to 10.
ndarry 7D               time:   [771.93 ms 775.16 ms 778.87 ms]
Found 10 outliers among 100 measurements (10.00%)
  4 (4.00%) high mild
  6 (6.00%) high severe
```

micro_ndarray is very consistent even with higher dimensions, while ndarray is extremely fast in 2D and exteremely slow in 7D
