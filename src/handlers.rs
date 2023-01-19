use crate::io;
use crate::sql;
use crate::schema;
use clap::ArgMatches;
use polars_lazy::prelude::LazyFrame;

pub fn handle_csv(matches: &ArgMatches) {
    let delimiter = match matches.get_one::<String>("delimiter") {
        Some(delimiter) => delimiter.as_bytes()[0],
        None => b',',
    };
    let mut ldf = if matches.get_flag("stdin") {
        io::load_csv_from_stdin(delimiter)
    } else {
        let path = matches
            .get_one::<String>("path")
            .expect("Please, provide a file");
        io::read_csv(path.to_string(), delimiter)
    };
    if let Some(query) = matches.get_one::<String>("query") {
        ldf = sql::execute(ldf, query);
    }
    if matches.get_flag("summary") {
        let df = ldf.collect().expect("Could not collect");
        println!("{}", df.describe(None));
    } else if matches.get_flag("head") {
        let df = ldf.fetch(5).expect("Could not fetch");
        println!("{}", df)
    } else {
        if matches.get_flag("text") {
            io::dump_csv_to_stdout(ldf);
        } else {
            if let Some(path) = matches.get_one::<String>("parquet") {
                io::write_parquet(ldf, path.to_string());
            } else {
                io::write_ipc(ldf);
            }
        }
    }
}

pub fn handle_sql(matches: &ArgMatches) {
    let delimiter = match matches.get_one::<String>("delimiter") {
        Some(delimiter) => delimiter.as_bytes()[0],
        None => b',',
    };
    if let Some(statement) = matches.get_one::<String>("statement") {
        let ldf = if matches.get_flag("text") {
            io::load_csv_from_stdin(delimiter)
        } else {
            io::read_ipc()
        };
        let res = sql::execute(ldf, statement);
        io::write_ipc(res);
    } else {
        io::write_ipc(io::read_ipc());
    }
}

pub fn handle_print(matches: &ArgMatches) {
    let delimiter = match matches.get_one::<String>("delimiter") {
        Some(delimiter) => delimiter.as_bytes()[0],
        None => b',',
    };
    let df = if matches.get_flag("text") {
        io::load_csv_from_stdin(delimiter)
    } else {
        io::read_ipc()
    };
    println!("{}", df.collect().expect("Could not collect"));
}

pub fn handle_rpq(matches: &ArgMatches) {
    let mut ldf = LazyFrame::default();
    if matches.get_flag("stdin") {
        ldf = io::load_parquet_from_stdin();
    } else if let Some(path) = matches.get_one::<String>("path") {
        ldf = io::read_parquet(path.to_string());
    } else {
        eprintln!("File not found or not reading from stdin")
    }
    if let Some(query) = matches.get_one::<String>("query") {
        ldf = sql::execute(ldf, query);
    }
    if matches.get_flag("summary") {
        let df = ldf.collect().expect("Could not collect");
        println!("{}", df.describe(None));
    } else if matches.get_flag("head") {
        let df = ldf.fetch(5).expect("Could not fetch");
        println!("{}", df)
    } else {
        if matches.get_flag("text") {
            io::dump_csv_to_stdout(ldf);
        } else {
            if let Some(path) = matches.get_one::<String>("parquet") {
                io::write_parquet(ldf, path.to_string());
            } else {
                io::write_ipc(ldf);
            }
        }
    }

}

pub fn handle_wpq(matches: &ArgMatches) {
    let delimiter = match matches.get_one::<String>("delimiter") {
        Some(delimiter) => delimiter.as_bytes()[0],
        None => b',',
    };
    if let Some(path) = matches.get_one::<String>("path") {
        let ldf = if matches.get_flag("text") {
            io::load_csv_from_stdin(delimiter)
        } else {
            io::read_ipc()
        };
        io::write_parquet(ldf, path.to_string());
    } else {
        eprintln!("Could now write to parquet");
    }
}

pub fn handle_schema(matches: &ArgMatches) {
    let delimiter = match matches.get_one::<String>("delimiter") {
        Some(delimiter) => delimiter.as_bytes()[0],
        None => b',',
    };
    let ldf = if matches.get_flag("stdin") {
        io::load_csv_from_stdin(delimiter)
    } else {
        io::read_ipc()
    };

    if matches.get_flag("summary") {
        schema::print_schema(ldf);
    } else if matches.get_flag("postgresql") {
        let name = matches
            .get_one::<String>("name")
            .expect("Please provide a table name");
        let strlen: u32 = match matches.get_one::<String>("strlen") {
            Some(strlen) => strlen.parse::<u32>().unwrap(),
            None => 128,
        };
        schema::print_create(ldf, name.as_str(), strlen);
    }
}