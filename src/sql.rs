use polars::frame::DataFrame;
use polars::prelude::*;
use std::io;
use std::io::Read;

/// Read from stdin from CSV format and return a Polars DataFrame
pub fn load_csv_from_stdin() -> PolarsResult<DataFrame> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let cursor = io::Cursor::new(buffer.as_bytes());
    CsvReader::new(cursor).finish()
}

/// Take a Polars Dataframe and write it as CSV to stdout
pub fn dump_csv_to_stdout(df: &mut DataFrame) -> Result<(), PolarsError> {
    CsvWriter::new(io::stdout().lock()).finish(df)
}
