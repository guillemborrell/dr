use polars::frame::DataFrame;
use polars::prelude::*;
use std::fs;
use std::io;
use std::io::Read;

/// Read CSV format from stdin and return a Polars DataFrame
pub fn load_csv_from_stdin() -> DataFrame {
    let mut buffer = String::new();
    let _res: () = match io::stdin().read_to_string(&mut buffer) {
        Ok(_ok) => (),
        Err(_e) => (),
    };
    let cursor = io::Cursor::new(buffer.as_bytes());
    let df = match CsvReader::new(cursor).finish() {
        Ok(df) => df,
        Err(_e) => DataFrame::default(),
    };
    df
}

/// Take a Polars Dataframe and write it as CSV to stdout
pub fn dump_csv_to_stdout(df: &mut DataFrame) {
    let _res: () = match CsvWriter::new(io::stdout().lock()).finish(df) {
        Ok(_ok) => (),
        Err(_e) => (),
    };
}

/// Read parquet and return a Polars DataFrame
pub fn read_parquet(path: String) -> DataFrame {
    let file = fs::File::open(path).expect("Could not open file");
    let df = match ParquetReader::new(file).finish() {
        Ok(df) => df,
        Err(e) => {
            eprintln!("{e}");
            DataFrame::default()
        }
    };
    df
}
