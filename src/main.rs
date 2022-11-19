mod sql;
use clap::{arg, command, Command};
use polars::prelude::*;

fn main() {
    let matches = command!()
        .subcommand(
            Command::new("sql")
                .about("Runs a sql statement on the file")
                .arg(arg!([statement] "SQL statement"))
                .arg(arg!(-d --delimiter <String> "Column delimiter").required(false)),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("sql") {
        if let Some(delimiter) = matches.get_one::<String>("delimiter") {
            println!("Delimiter: {delimiter}")
        } else {
            println!("No delimiter")
        }
        if let Some(statement) = matches.get_one::<String>("statement") {
            println!("Statement: {statement}");
        } else {
            let mut df = match sql::load_csv_from_stdin() {
                Ok(df) => df,
                Err(_e) => DataFrame::default(),
            };
            let _res = match sql::dump_csv_to_stdout(&mut df) {
                Ok(_df) => (),
                Err(_e) => (),
            };
        }
    }
}
