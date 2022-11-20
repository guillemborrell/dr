# csvgr.rs

A set of csv files processing utilities inspired by [csvkit](https://github.com/wireservice/csvkit) with blazing speed, powered by Rust.

You may wonder why I'm implementing this, since there's already [xsv](https://github.com/BurntSushi/xsv). There are two reasons for that:

1. This is the project I'm implementing to learn Rust
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

$ cat wine.csv  | ./target/release/csvgr sql "select Wine, avg(Alcohol) from this group by Wine" | ./target/release/csvgr print
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

## Built standing on the shoulders of giants

None of this would be possible without [Polars](https://github.com/pola-rs/polars)