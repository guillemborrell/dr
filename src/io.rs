use polars::prelude::*;
use std::fs;
use std::io;
use std::io::Read;

/// Read CSV file
pub fn read_csv(path: String) -> LazyFrame {
    LazyCsvReader::new(path)
        .finish()
        .expect("Could not load file")
}

/// Read parquet and return a Polars LazyFrame
pub fn read_parquet(path: String) -> LazyFrame {
    LazyFrame::scan_parquet(path, ScanArgsParquet::default()).expect("Could not read parquet file")
}

/// Read IPC setream
pub fn read_ipc() -> LazyFrame {
    let mut buffer = Vec::new();
    let _res: () = match io::stdin().lock().read_to_end(&mut buffer) {
        Ok(_ok) => (),
        Err(_e) => (),
    };
    let cursor = io::Cursor::new(buffer);
    match IpcStreamReader::new(cursor).finish() {
        Ok(df) => df.lazy(),
        Err(_e) => LazyFrame::default(),
    }
}

/// Read CSV format from stdin and return a Polars DataFrame
pub fn load_csv_from_stdin() -> LazyFrame {
    let mut buffer = Vec::new();
    let _res: () = match io::stdin().lock().read_to_end(&mut buffer) {
        Ok(_ok) => (),
        Err(_e) => (),
    };
    let cursor = io::Cursor::new(buffer);
    match CsvReader::new(cursor).finish() {
        Ok(df) => df.lazy(),
        Err(_e) => LazyFrame::default(),
    }
}

/// Write to IPC steram
pub fn write_ipc(df: LazyFrame) {
    IpcStreamWriter::new(io::stdout().lock())
        .finish(&mut df.collect().expect("Could not collect dataframe"))
        .expect("Could not write to stream");
}

/// Take a Polars Dataframe and write it as CSV to stdout
pub fn dump_csv_to_stdout(df: &mut DataFrame) {
    let _res: () = match CsvWriter::new(io::stdout().lock()).finish(df) {
        Ok(_ok) => (),
        Err(_e) => (),
    };
}

/// Write a Polars DataFrame to Parquet
pub fn write_parquet(
    mut df: DataFrame,
    path: String,
    compression: String,
    statistics: bool,
    chunksize: Option<usize>,
) {
    // Selected compression not implemented yet
    let mut _file = match fs::File::create(path) {
        Ok(mut file) => {
            let mut w = ParquetWriter::new(&mut file);
            if statistics {
                w = w.with_statistics(statistics);
            }
            if chunksize.unwrap_or(0) > 0 {
                w = w.with_row_group_size(chunksize);
            }
            let _r = match w.finish(&mut df) {
                Ok(_r) => (),
                Err(e) => eprintln!("{e}"),
            };
        }
        Err(e) => eprintln!("{e}"),
    };
}
