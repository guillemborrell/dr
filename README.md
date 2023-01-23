# dr.rs

[![status-badge](https://ci.guillemborrell.es/api/badges/guillem/dr/status.svg)](https://ci.guillemborrell.es/guillem/dr) | [Download](https://git.guillemborrell.es/guillem/-/packages/generic/dr) | [Source](https://git.guillemborrell.es/guillem/dr) | [Bugs](https://github.com/guillemborrell/dr)

A toolkit to process data files (csv and parquet) using the command line, inspired by [csvkit](https://github.com/wireservice/csvkit), with blazing speed, and powered by Rust.

You may wonder why I'm implementing this, since there's already [xsv](https://github.com/BurntSushi/xsv). There are two reasons for that:

1. This what I'm implementing to learn Rust.
2. The Rust data ecosystem has evolved immensely since xsv was sarted. Now we can add things like SQL commands to filter csv files, or translate results to parquet files.


## TL;DR

You can install dr the rust way with `cargo install dr` but downloading a binary from [here](https://git.guillemborrell.es/guillem/-/packages/generic/dr) may be all you need.

```
$ dr --help
dr is a handy command line tool to handle csv and parquet files.
It is designed to integrate nicely with other command line tools
like cat, sed, awk and database clients cli. You can find more
information an a short tutorial https://git.guillemborrell.es/guillem/dr
            

Usage: dr [COMMAND]

Commands:
  csv
          Read csv, output arrow stream
  schema
          Several table schema related utilities
  sql
          Runs a sql statement on the file
  print
          Pretty prints the table
  rpq
          Read parquet file
  wpq
          Write to a paquet file
  help
          Print this message or the help of the given subcommand(s)

Options:
  -h, --help
          Print help information (use `-h` for a summary)

  -V, --version
          Print version information
```

## Howto

`dr` is convenience command to explore, transform, and analyze csv and parquet files to save you from writing throwaway python scripts or create a custom container image for verys simple tasks. It's designed to make the life of a data engineer a little easier.

Assume you have a very large csv file, and you just want to translate it to parquet with some type inference and sane defaults. With `dr` this is as easy as:

```
$ dr csv wine.csv -P wine.pq
```

Parquet files are binary, and you may want to check that you've not written nonsense by printing the header on your terminal.

```
$ dr rpq wine.pq -a
shape: (5, 14)
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
├╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 1    ┆ 13.24   ┆ 2.59       ┆ 2.87 ┆ ... ┆ 4.32      ┆ 1.04 ┆ 2.93 ┆ 735     │
└──────┴─────────┴────────────┴──────┴─────┴───────────┴──────┴──────┴─────────┘
```

Maybe the most interesing feature of `dr` is the ability to process csv and parquet files using SQL, while solutions like `xsv` and `csvkit` rely on a rich set of subcommands and options. If you already know SQL, there's no need to read any more documentation to select, filter, or group data. The only thing you need to remember is that the table will be called `this`. The following command outputs a csv of the wine with the highest concentration of alcohol in the popular wine dataset:

```
 dr csv wine.csv -q "select * from this where Alcohol = max(Alcohol)" | dr print
shape: (1, 14)
┌──────┬─────────┬────────────┬──────┬─────┬───────────┬──────┬──────┬─────────┐
│ Wine ┆ Alcohol ┆ Malic.acid ┆ Ash  ┆ ... ┆ Color.int ┆ Hue  ┆ OD   ┆ Proline │
│ ---  ┆ ---     ┆ ---        ┆ ---  ┆     ┆ ---       ┆ ---  ┆ ---  ┆ ---     │
│ i64  ┆ f64     ┆ f64        ┆ f64  ┆     ┆ f64       ┆ f64  ┆ f64  ┆ i64     │
╞══════╪═════════╪════════════╪══════╪═════╪═══════════╪══════╪══════╪═════════╡
│ 1    ┆ 14.83   ┆ 1.64       ┆ 2.17 ┆ ... ┆ 5.2       ┆ 1.08 ┆ 2.85 ┆ 1045    │
└──────┴─────────┴────────────┴──────┴─────┴───────────┴──────┴──────┴─────────┘
```

If you don't use any option that formats the output of the results, `dr` outputs Arrow's IPC format, meaning that multiple `dr` calls can be efficiently chained with very low overhead. The following script loads one month of NY taxi data and executes two sql queries on the data. 

```
$ dr rpq data/yellow_tripdata_2014-01.parquet \
    -q "select count(1) as cnt, passenger_count from this group by passenger_count" \
    | dr sql "select * from this order by cnt desc" \
    | dr print
┌─────────┬─────────────────┐
│ cnt     ┆ passenger_count │
│ ---     ┆ ---             │
│ u32     ┆ i64             │
╞═════════╪═════════════════╡
│ 9727321 ┆ 1               │
├╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 1891588 ┆ 2               │
├╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 789070  ┆ 5               │
├╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 566248  ┆ 3               │
├╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ ...     ┆ ...             │
├╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 19      ┆ 208             │
├╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 16      ┆ 9               │
├╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 7       ┆ 7               │
├╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 5       ┆ 8               │
└─────────┴─────────────────┘
```

### Operate with SQL databases

How many times did you have to insert a csv file (sometimes larger than memory) to a database? Tens of times? Hundreds? You've probably used Pandas for that, since it can infer the table's datatypes. So a simple data operation becomes a python script with Pandas and a driver for PostgreSQL as dependencies.

Now dr can provide the table creation statement with a handful of columns:

```
$ head wine.csv | dr schema -i -p -n wine
CREATE TABLE IF NOT EXISTS "wine" (  );
ALTER TABLE "wine" ADD COLUMN "Wine" integer;
ALTER TABLE "wine" ADD COLUMN "Alcohol" real;
ALTER TABLE "wine" ADD COLUMN "Malic.acid" real;
ALTER TABLE "wine" ADD COLUMN "Ash" real;
ALTER TABLE "wine" ADD COLUMN "Acl" real;
ALTER TABLE "wine" ADD COLUMN "Mg" integer;
ALTER TABLE "wine" ADD COLUMN "Phenols" real;
ALTER TABLE "wine" ADD COLUMN "Flavanoids" real;
ALTER TABLE "wine" ADD COLUMN "Nonflavanoid.phenols" real;
ALTER TABLE "wine" ADD COLUMN "Proanth" real;
ALTER TABLE "wine" ADD COLUMN "Color.int" real;
ALTER TABLE "wine" ADD COLUMN "Hue" real;
ALTER TABLE "wine" ADD COLUMN "OD" real;
ALTER TABLE "wine" ADD COLUMN "Proline" integer;
```

If you're fine with dr's choices you can then create the table and insert the file

```
$ head wine.csv | dr schema -i -p -n wine | psql
$ cat wine.csv | psql -c "\copy wine from stdin with (FORMAT 'csv', HEADER)"
```

Since most databases can ingest and spit CSV files, some simple operations can be enhanced with dr, like storing the results of a query in a parquet file

```
$ psql -c "copy (select * from wine) to stdout with (FORMAT 'csv', HEADER)" | dr csv -i -P wine.pq
```

## Reference

Some commands that generate raw output in ipc format.

* Read a csv or parquet file and print the header: `dr {csv, rpq} [file] -a`
* Read a csv or parquet file, execute a SQL statement, and output the results in stdout using Arrow's ipc format `dr {csv, rpq} [file] -q "statement"`
* Read a csv or parquet file and print a summary of each column: `dr {csv, rpq} [file] -s "[query]"`
* Read a csv or parquet file, execute a query, and output the results in stdout using the csv format `dr {csv, rpq} [file] -s "[query]" -t`
* Read a csv and write a parquet file with the same contents: `dr csv [file.csv] -P [file.pq]`

Some commands that convert raw input in ipc format

* Read from stdin in ipc and pretty print the table: `dr print`
* Read from stdin in csv and pretty print the table: `dr print -t`
* Read from stdin in ipc and write the data in parquet: `dr wpq [file.pq]` 

## Performance

This command runs two dr processes. The first one makes an aggregation on a compressed parquet file of 144MB of size, and the second one just orders the result:

```
$ dr rpq data/yellow_tripdata_2014-01.parquet \
    -q "select count(1) as cnt, passenger_count from this group by passenger_count" \
    | dr sql "select * from this order by cnt desc" \
    > /dev/null
```

On a very very old machine (Intel(R) Core(TM) i5-6500T CPU @ 2.50GHz), this takes around half a second, which is roughly the time needed to read and decompress the parquet file. Polar's csv and parquet readers have some decent performance, so you can count on `dr` to be one of the fastest in the block.

## Caveats

1. `dr` uses Polars to build and transform dataframes in Rust, and the entire table may be loaded in memory. At the time when `dr` was created, streaming support didn't get along very well with SQL contexts.

2. `dr` uses Polars' SQLContext to execute the query which supports a small subset of the SQL language.

## Built standing on the shoulders of giants

None of this would be possible without [Polars](https://github.com/pola-rs/polars)