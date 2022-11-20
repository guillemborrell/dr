mod io;
mod sql;
use clap::{arg, command, Command};

fn main() {
    let matches = command!()
        .subcommand(
            Command::new("sql")
                .about("Runs a sql statement on the file")
                .arg(arg!([statement] "SQL statement"))
                .arg(arg!(-d --delimiter <String> "Column delimiter").required(false)),
        )
        .subcommand(Command::new("print").about("Pretty prints the table"))
        .subcommand(Command::new("rpq").about("Read parquet file"))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("sql") {
        //if let Some(delimiter) = matches.get_one::<String>("delimiter") {
        //    println!("DEBUG: Delimiter: {delimiter}")
        //} else {
        //    println!("DEBUG: No delimiter")
        //}
        if let Some(statement) = matches.get_one::<String>("statement") {
            sql::execute(statement);
        } else {
            let mut df = io::load_csv_from_stdin();
            io::dump_csv_to_stdout(&mut df);
        }
    }

    if let Some(_matches) = matches.subcommand_matches("print") {
        let df = io::load_csv_from_stdin();
        println!("{}", df)
    }

    if let Some(_matches) = matches.subcommand_matches("rpq") {
        let mut df = io::load_parquet_from_stdin();
        io::dump_csv_to_stdout(&mut df);
    }
}
