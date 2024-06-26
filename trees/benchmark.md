# Benchmark
## Small
| Test              | Gears                      | Go           | Ratio                               |
| :---------------- | :------------------------- | :----------  | :---------------------------------- |
| Query miss (fast) |                            | 589ns           |      |
| Query miss (slow) |  1.442µs | 1.617µs           | <mark style="background-color: green">&nbsp;0.9&nbsp;</mark>                                     |
| Query hit (fast)  |                            | 61ns           |       |
| Query hit (slow)  |  1.733µs  | 2.96µs           | <mark style="background-color: green">&nbsp;0.6&nbsp;</mark>                                     |
| Iter (fast)       |                            | 505.801µs           |            |
| Iter (slow)       | 1.535414ms        | 2.181263ms           | <mark style="background-color: green">&nbsp;0.7&nbsp;</mark>                                     |
| Update            |  29.912µs     | 29.918µs           | <mark style="background-color: green">&nbsp;1.0&nbsp;</mark>          |
| Run Blocks        |  9.820595ms | 7.348834ms           | <mark style="background-color: red">&nbsp;1.3&nbsp;</mark>      |
## Medium
| Test              | Gears                      | Go           | Ratio                               |
| :---------------- | :------------------------- | :----------  | :---------------------------------- |
| Query miss (fast) |                            | 2.34µs           |      |
| Query miss (slow) |  5.427µs | 9.099µs           | <mark style="background-color: green">&nbsp;0.6&nbsp;</mark>                                     |
| Query hit (fast)  |                            | 406ns           |       |
| Query hit (slow)  |  6.653µs  | 12.909µs           | <mark style="background-color: green">&nbsp;0.5&nbsp;</mark>                                     |
| Iter (fast)       |                            | 41.978635ms           |            |
| Iter (slow)       | 425.488201ms        | 964.896104ms           | <mark style="background-color: green">&nbsp;0.4&nbsp;</mark>                                     |
| Update            |  132.048µs     | 116.014µs           | <mark style="background-color: red">&nbsp;1.1&nbsp;</mark>          |
| Run Blocks        |  15.485151ms | 16.063524ms           | <mark style="background-color: green">&nbsp;1.0&nbsp;</mark>      |
## Large
| Test              | Gears                      | Go           | Ratio                               |
| :---------------- | :------------------------- | :----------  | :---------------------------------- |
| Query miss (fast) |                            | 5.139µs           |      |
| Query miss (slow) |  26.112µs | 17.639µs           | <mark style="background-color: red">&nbsp;1.5&nbsp;</mark>                                     |
| Query hit (fast)  |                            | 5.339µs           |       |
| Query hit (slow)  |  28.99µs  | 23.944µs           | <mark style="background-color: red">&nbsp;1.2&nbsp;</mark>                                     |
| Iter (fast)       |                            | 651.533418ms           |            |
| Iter (slow)       | 17.807431921s        | 8.784634345s           | <mark style="background-color: red">&nbsp;2.0&nbsp;</mark>                                     |
| Update            |  194.715µs     | 242.246µs           | <mark style="background-color: green">&nbsp;0.8&nbsp;</mark>          |
| Run Blocks        |  18.05446ms | 54.795291ms           | <mark style="background-color: green">&nbsp;0.3&nbsp;</mark>      |
