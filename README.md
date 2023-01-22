# Double random

thread_rng double_random_unsafe
                        time:   [44.779 ns 45.004 ns 45.310 ns]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

thread_rng double_random_safe
                        time:   [48.465 ns 48.669 ns 48.913 ns]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

chacha20 double_random_unsafe
                        time:   [52.094 ns 52.189 ns 52.286 ns]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

chacha20 double_random_safe
                        time:   [56.111 ns 56.359 ns 56.624 ns]
Found 8 outliers among 100 measurements (8.00%)
  8 (8.00%) high mild
