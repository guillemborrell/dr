use polars_io::prelude::*;
use polars_lazy::prelude::*;
use std::io;
use std::io::Read;
use std::path::PathBuf;

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
pub fn dump_csv_to_stdout(ldf: LazyFrame) {
    let _res: () = match CsvWriter::new(io::stdout().lock())
        .finish(&mut ldf.collect().expect("Could not collect"))
    {
        Ok(_ok) => (),
        Err(_e) => (),
    };
}

/// Write a Polars DataFrame to Parquet
pub fn write_parquet(ldf: LazyFrame, path: String) {
    // Selected compression not implemented yet
    let mut p = PathBuf::new();
    p.push(path);
    ldf.sink_parquet(
        p,
        ParquetWriteOptions {
            compression: ParquetCompression::Snappy,
            statistics: true,
            row_group_size: None,
            data_pagesize_limit: None,
            maintain_order: false,
        },
    ).expect("Could not save");
}
