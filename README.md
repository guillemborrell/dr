# dr.rs

[![status-badge](https://ci.guillemborrell.es/api/badges/guillem/dr/status.svg)](https://ci.guillemborrell.es/guillem/dr) | [Download](https://git.guillemborrell.es/guillem/-/packages/generic/dr)

A toolkit to process data files (csv and parquet) using the command line, inspired by [csvkit](https://github.com/wireservice/csvkit), with blazing speed, and powered by Rust.

You may wonder why I'm implementing this, since there's already [xsv](https://github.com/BurntSushi/xsv). There are two reasons for that:

1. This what I'm implementing to learn Rust
2. The Rust data ecosystem has evolved immensely since xsv was sarted. Now we can add things like SQL commands to filter csv files, or translate results to parquet files.

## Example

```bash
$ head wine.csv
Wine,Alcohol,Malic.acid,Ash,Acl,Mg,Phenols,Flavanoids,Nonflavanoid.phenols,Proanth,Color.int,Hue,OD,Proline
1,14.23,1.71,2.43,15.6,127,2.8,3.06,.28,2.29,5.64,1.04,3.92,1065
1,13.2,1.78,2.14,11.2,100,2.65,2.76,.26,1.28,4.38,1.05,3.4,1050
1,13.16,2.36,2.67,18.6,101,2.8,3.24,.3,2.81,5.68,1.03,3.17,1185
1,14.37,1.95,2.5,16.8,113,3.85,3.49,.24,2.18,7.8,.86,3.45,1480
1,13.24,2.59,2.87,21,118,2.8,2.69,.39,1.82,4.32,1.04,2.93,735
1,14.2,1.76,2.45,15.2,112,3.27,3.39,.34,1.97,6.75,1.05,2.85,1450
1,14.39,1.87,2.45,14.6,96,2.5,2.52,.3,1.98,5.25,1.02,3.58,1290
1,14.06,2.15,2.61,17.6,121,2.6,2.51,.31,1.25,5.05,1.06,3.58,1295
1,14.83,1.64,2.17,14,97,2.8,2.98,.29,1.98,5.2,1.08,2.85,1045

$ cat wine.csv | dr sql "select Wine, avg(Alcohol) from this group by Wine" | dr print
shape: (3, 2)
┌──────┬───────────┐
│ Wine ┆ Alcohol   │
│ ---  ┆ ---       │
│ i64  ┆ f64       │
╞══════╪═══════════╡
│ 3    ┆ 13.15375  │
├╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
│ 1    ┆ 13.744746 │
├╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
│ 2    ┆ 12.278732 │
└──────┴───────────┘
```

## Howto

The `dr` command offers a set of subcommands, each one of them with a different functionality. You can get the available subcommands with:

```bash
$ dr --help
Command-line data file processing in Rust

Usage: dr [COMMAND]

Commands:
  sql    Runs a sql statement on the file
  print  Pretty prints the table
  rpq    Read parquet file
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

Subcommands can be pipelined unless reading from a file, writing to a file, or pretty prints data. What goes through the pipeline is a plain-text comma separated values with a header. While this may not be the best choice in terms of performance, allows `dr` subcommands to be combined with the usual unix-style command-line tools like `cat`, `head`, `grep`, `awk` and `sed`:

```bash
$ cat wine.csv | head -n 5 | dr print
shape: (4, 14)
┌──────┬─────────┬────────────┬──────┬─────┬───────────┬──────┬──────┬─────────┐
│ Wine ┆ Alcohol ┆ Malic.acid ┆ Ash  ┆ ... ┆ Color.int ┆ Hue  ┆ OD   ┆ Proline │
│ ---  ┆ ---     ┆ ---        ┆ ---  ┆     ┆ ---       ┆ ---  ┆ ---  ┆ ---     │
│ i64  ┆ f64     ┆ f64        ┆ f64  ┆     ┆ f64       ┆ f64  ┆ f64  ┆ i64     │
╞══════╪═════════╪════════════╪══════╪═════╪═══════════╪══════╪══════╪═════════╡
│ 1    ┆ 14.23   ┆ 1.71       ┆ 2.43 ┆ ... ┆ 5.64      ┆ 1.04 ┆ 3.92 ┆ 1065    │
├╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 1    ┆ 13.2    ┆ 1.78       ┆ 2.14 ┆ ... ┆ 4.38      ┆ 1.05 ┆ 3.4  ┆ 1050    │
├╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 1    ┆ 13.16   ┆ 2.36       ┆ 2.67 ┆ ... ┆ 5.68      ┆ 1.03 ┆ 3.17 ┆ 1185    │
├╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 1    ┆ 14.37   ┆ 1.95       ┆ 2.5  ┆ ... ┆ 7.8       ┆ 0.86 ┆ 3.45 ┆ 1480    │
└──────┴─────────┴────────────┴──────┴─────┴───────────┴──────┴──────┴─────────┘
```

Note that when `dr` loads csv data also tries to guess the data type of each field.


## Performance

`dr` is implemented in Rust with the goal of achieving the highest possible performance. Take for instance a simple read, groupby, and aggregate operation with ~30MB of data:

```bash
$ time cat data/walmart_train.csv | ./target/release/dr sql "select Dept, avg("Weekly_Sales") from this group by Dept" | ./target/release/dr print
shape: (81, 2)
┌──────┬──────────────┐
│ Dept ┆ Weekly_Sales │
│ ---  ┆ ---          │
│ i64  ┆ f64          │
╞══════╪══════════════╡
│ 30   ┆ 4118.197208  │
├╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 16   ┆ 14245.63827  │
├╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 56   ┆ 3833.706211  │
├╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 24   ┆ 6353.604562  │
├╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ ...  ┆ ...          │
├╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 31   ┆ 2339.440287  │
├╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 59   ┆ 694.463564   │
├╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 27   ┆ 1583.437727  │
├╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 77   ┆ 328.9618     │
└──────┴──────────────┘

real    0m0.089s
user    0m0.116s
sys     0m0.036s
```

Let's compare that with the followint Python script that leverages Pandas to read the data, and compute the aggregation:

```python
#!/usr/bin/env python3

import sys
import pandas as pd

df = pd.read_csv(sys.stdin)
print(df.groupby("Dept", sort=False, as_index=False).Weekly_Sales.mean())
```

```bash
$ time cat data/walmart_train.csv | ./python/group.py
    Dept  Weekly_Sales
0      1  19213.485088
1      2  43607.020113
2      3  11793.698516
3      4  25974.630238
4      5  21365.583515
..   ...           ...
76    99    415.487065
77    39     11.123750
78    50   2658.897010
79    43      1.193333
80    65  45441.706224

[81 rows x 2 columns]

real    0m0.717s
user    0m0.627s
sys     0m0.282s
```

Note that there's roughly a 6x speedup. This considering that this operation in particular is heavily optimized in Pandas and most of the run time is spent in parsing and reading from stdin.


## Built standing on the shoulders of giants

None of this would be possible without [Polars](https://github.com/pola-rs/polars)