# LCS_RS
This project is a rust implementation of a solution to the following problem:


Given $`N`$ binary files of combined total length $`L`$, find the longest common binary subsequence which is present in at least $`K`$ of those files.

This implementation uses a suffix array construction algorithm based on that in [rust-bio](https://github.com/rust-bio/rust-bio), 
as well as several ideas from the algorithm presented in [^fn1].

### Usage
Clone the repo, and run `cargo build --release` in order to build an optimized binary.

Example usage: 
```
$ lcs_rs -k 2 file_1 file_2 file_3 file_4
LCS found with length 27648
-> in sample.2 at 3072
-> in sample.3 at 17408
```

### Benchmarks

Varying K:
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `lcs_rs -k 2 sample.1 ... sample.10` | 84.3 ± 5.2 | 78.4 | 103.8 | 1.00 |
| `lcs_rs -k 3 sample.1 ... sample.10` | 97.5 ± 3.1 | 91.8 | 104.3 | 1.16 ± 0.08 |
| `lcs_rs -k 4 sample.1 ... sample.10` | 107.3 ± 3.0 | 103.1 | 113.9 | 1.27 ± 0.09 |
| `lcs_rs -k 5 sample.1 ... sample.10` | 121.2 ± 3.7 | 115.5 | 129.6 | 1.44 ± 0.10 |
| `lcs_rs -k 6 sample.1 ... sample.10` | 135.3 ± 3.1 | 128.9 | 141.9 | 1.61 ± 0.11 |
| `lcs_rs -k 7 sample.1 ... sample.10` | 147.8 ± 6.1 | 141.7 | 169.2 | 1.75 ± 0.13 |
| `lcs_rs -k 8 sample.1 ... sample.10` | 164.6 ± 6.5 | 158.3 | 186.9 | 1.95 ± 0.14 |
| `lcs_rs -k 9 sample.1 ... sample.10` | 183.7 ± 2.8 | 178.1 | 189.1 | 2.18 ± 0.14 |

Varying the number of files: 
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `lcs_rs -k 2 sample.1 ... sample.2` | 9.2 ± 1.3 | 7.8 | 14.4 | 1.00 |
| `lcs_rs -k 2 sample.1 ... sample.3` | 23.2 ± 1.2 | 21.7 | 27.5 | 2.52 ± 0.38 |
| `lcs_rs -k 2 sample.1 ... sample.4` | 33.0 ± 1.2 | 31.3 | 38.4 | 3.59 ± 0.53 |
| `lcs_rs -k 2 sample.1 ... sample.5` | 40.8 ± 1.7 | 38.5 | 47.7 | 4.43 ± 0.66 |
| `lcs_rs -k 2 sample.1 ... sample.6` | 51.4 ± 2.0 | 48.7 | 57.8 | 5.58 ± 0.82 |
| `lcs_rs -k 2 sample.1 ... sample.7` | 60.2 ± 2.5 | 56.5 | 68.7 | 6.54 ± 0.97 |
| `lcs_rs -k 2 sample.1 ... sample.8` | 69.5 ± 2.7 | 65.6 | 76.4 | 7.55 ± 1.11 |
| `lcs_rs -k 2 sample.1 ... sample.9` | 75.8 ± 2.9 | 72.0 | 82.8 | 8.23 ± 1.21 |
| `lcs_rs -k 2 sample.1 ... sample.10` | 80.4 ± 2.6 | 77.2 | 90.8 | 8.73 ± 1.27 |

We see very linear scaling with each! $`\mathcal{O}(n)`$ acheived!

### Notes on implementation

The basic structure of this implementation is as follows:

1. Read in all files as `u8` bytes
2. Increment all bytes by 1 into `u16`s to make room for a "sentinel", `0u16`.
3. Append all files terminated with the sentinel
4. Use the custom `u16` implementation of [SAIS](https://zork.net/~st/jottings/sais.html) to construct the suffix array from the combined files
5. Build the Longest Common Prefix array
6. Scan across the suffix array for subsequences which are present in at least $`K`$ of the files
7. Compute the minimum LCP for all prefixes in those subseqences
8. Find the subsequence with the maximum min LCP
9. Determine and return the resulting files present in that subsequence of the suffix array with their offsets

This is the first thing I've ever written in rust, so the code may be a little sloppy! 
Feel free to make an issue if something could be written in a better way. 

##### References
[^fn1]: Babenko M.A., Starikovskaya T.A. (2008) Computing Longest Common Substrings Via Suffix Arrays. In: Hirsch E.A., Razborov A.A., Semenov A., Slissenko A. (eds) Computer Science – Theory and Applications. CSR 2008. Lecture Notes in Computer Science, vol 5010. Springer, Berlin, Heidelberg
