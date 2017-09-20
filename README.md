# raiders-of-lost-ark
A CLI to search from files and directories. A mini version of grep

[![Build Status](https://travis-ci.org/addityasingh/raiders-of-lost-ark.svg?branch=master)](https://travis-ci.org/addityasingh/raiders-of-lost-ark)

## Features
- Supports finding all lines with given `query` word
- Find CASE_INSENSITVE match
- Highlight the search result
- Find count of all occurrences

![highlight for search result](https://github.com/addityasingh/raiders-of-lost-ark/blob/master/images/syntax-highlight.png)

## Benchmarks
```bash
cargo bench
```

The result of running the above on the benchmark test in benches is

```bash
running 2 tests
test tests::bench_search_empty    ... bench:         280 ns/iter (+/- 119)
test tests::bench_search_nonempty ... bench:         814 ns/iter (+/- 401)
```
