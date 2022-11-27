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
        .subcommand(
            Command::new("rpq")
                .about("Read parquet file")
                .arg(arg!([path] "Path to the parquet file")),
        )
        .subcommand(
            Command::new("wpq")
                .about("Write to a paquet file")
                .arg(arg!([path] "Path to the new parquet file")),
        )
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
    } else if let Some(_matches) = matches.subcommand_matches("print") {
        let df = io::load_csv_from_stdin();
        println!("{}", df)
    } else if let Some(matches) = matches.subcommand_matches("rpq") {
        if let Some(path) = matches.get_one::<String>("path") {
            let mut df = io::read_parquet(path.to_string());
            io::dump_csv_to_stdout(&mut df);
        } else {
            eprintln!("File not found")
        }
    } else if let Some(matches) = matches.subcommand_matches("wpq") {
        if let Some(path) = matches.get_one::<String>("path") {
            let df = io::load_csv_from_stdin();
            io::write_parquet(df, path.to_string(), "lz4raw".to_string(), true, Some(0));
        } else {
            eprintln!("Could now write to parquet");
        }
    } else {
        println!("No command provided. Please execute dr --help")
    }
}
