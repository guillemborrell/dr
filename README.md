# csvgr.rs

A set of csv files processing utilities inspired by [csvkit](https://github.com/wireservice/csvkit) with blazing speed, powered by Rust.

You may wonder why I'm implementing this, since there's already [xsv](https://github.com/BurntSushi/xsv). There are two reasons for that:

1. This is the project I'm implementing to learn Rust
2. The Rust data ecosystem has evolved immensely since xsv was sarted. Now we can add things like SQL commands to filter csv files, or translate results to parquet files.