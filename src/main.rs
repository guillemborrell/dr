mod io;
mod schema;
mod sql;
use clap::{arg, command, ArgAction, Command};
use polars_lazy::prelude::*;

fn main() {
    let matches = command!()
        .subcommand(
            Command::new("csv")
                .about("Read csv, output arrow stream")
                .arg(arg!([path] "Path to CSV file"))
                .arg(arg!(-d --delimiter <String> "Column delimiter. Assume ,").required(false))
                .arg(
                    arg!(-i --stdin ... "Read from stdin")
                        .required(false)
                        .action(ArgAction::SetTrue),
                )
                .arg(arg!(-q --query <String> "Execute query on the file").required(false))
                .arg(
                    arg!(-s --summary ... "Summarize the data")
                        .required(false)
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    arg!(-t --text ... "Output text instead of binary")
                        .required(false)
                        .action(ArgAction::SetTrue),
                )
                .arg(arg!(-P --parquet <String> "Write output as a parquet file").required(false))
                .arg(
                    arg!(-a --head ... "Print the header of the table")
                        .required(false)
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("schema")
                .about("Several table schema related utilities")
                .arg(arg!(-n --name <String> "Table name").required(false))
                .arg(arg!(-l --strlen <String> "Default length for string columns").required(false))
                .arg(
                    arg!(-s --summary ... "Summarize the schema")
                        .required(false)
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    arg!(-p --postgresql ... "Create a postgresql table with schema")
                        .required(false)
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("sql")
                .about("Runs a sql statement on the file")
                .arg(arg!([statement] "SQL statement"))
                .arg(
                    arg!(-t --text ... "Input text instead of binary")
                        .required(false)
                        .action(ArgAction::SetTrue),
                )
                .arg(arg!(-d --delimiter <String> "Column delimiter").required(false)),
        )
        .subcommand(
            Command::new("print").about("Pretty prints the table").arg(
                arg!(-t --text ... "Inputs csv instead of binary")
                    .required(false)
                    .action(ArgAction::SetTrue),
            ),
        )
        .subcommand(
            Command::new("rpq")
                .about("Read parquet file")
                .arg(arg!([path] "Path to the parquet file"))
                .arg(arg!(-q --query <String> "Execute query on the file").required(false))
                .arg(
                    arg!(-s --summary ... "Summarize the data")
                        .required(false)
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    arg!(-i --stdin ... "Read from stdin instead than from a file")
                        .required(false)
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    arg!(-t --text ... "Output text instead of binary")
                        .required(false)
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    arg!(-P --parquet <String> "Write the result as a parquet file")
                        .required(false),
                )
                .arg(
                    arg!(-a --head ... "Print the header of the table")
                        .required(false)
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("wpq")
                .about("Write to a paquet file")
                .arg(
                    arg!(-t --text ... "Input text instead of binary")
                        .required(false)
                        .action(ArgAction::SetTrue),
                )
                .arg(arg!([path] "Path to the new parquet file")),
        )
        .get_matches();
    if let Some(_matches) = matches.subcommand_matches("csv") {
        let mut ldf = if _matches.get_flag("stdin") {
            io::load_csv_from_stdin()
        } else {
            let path = _matches
                .get_one::<String>("path")
                .expect("Please, provide a file");
            io::read_csv(path.to_string())
        };
        if let Some(query) = _matches.get_one::<String>("query") {
            ldf = sql::execute(ldf, query);
        }
        if _matches.get_flag("summary") {
            let df = ldf.collect().expect("Could not collect");
            println!("{}", df.describe(None));
        } else if _matches.get_flag("head") {
            let df = ldf.fetch(5).expect("Could not fetch");
            println!("{}", df)
        } else {
            if _matches.get_flag("text") {
                io::dump_csv_to_stdout(ldf);
            } else {
                if let Some(path) = _matches.get_one::<String>("parquet") {
                    io::write_parquet(ldf, path.to_string());
                } else {
                    io::write_ipc(ldf);
                }
            }
        }
    } else if let Some(_matches) = matches.subcommand_matches("sql") {
        if let Some(statement) = _matches.get_one::<String>("statement") {
            let ldf = if _matches.get_flag("text") {
                io::load_csv_from_stdin()
            } else {
                io::read_ipc()
            };
            let res = sql::execute(ldf, statement);
            io::write_ipc(res);
        } else {
            io::write_ipc(io::read_ipc());
        }
    } else if let Some(_matches) = matches.subcommand_matches("print") {
        let df = if _matches.get_flag("text") {
            io::load_csv_from_stdin()
        } else {
            io::read_ipc()
        };
        println!("{}", df.collect().expect("Could not collect"));
    } else if let Some(_matches) = matches.subcommand_matches("rpq") {
        let mut ldf = LazyFrame::default();
        if _matches.get_flag("stdin") {
            ldf = io::load_parquet_from_stdin();
        } else if let Some(path) = _matches.get_one::<String>("path") {
            ldf = io::read_parquet(path.to_string());
        } else {
            eprintln!("File not found or not reading from stdin")
        }
        if let Some(query) = _matches.get_one::<String>("query") {
            ldf = sql::execute(ldf, query);
        }
        if _matches.get_flag("summary") {
            let df = ldf.collect().expect("Could not collect");
            println!("{}", df.describe(None));
        } else if _matches.get_flag("head") {
            let df = ldf.fetch(5).expect("Could not fetch");
            println!("{}", df)
        } else {
            if _matches.get_flag("text") {
                io::dump_csv_to_stdout(ldf);
            } else {
                if let Some(path) = _matches.get_one::<String>("parquet") {
                    io::write_parquet(ldf, path.to_string());
                } else {
                    io::write_ipc(ldf);
                }
            }
        }
    } else if let Some(_matches) = matches.subcommand_matches("wpq") {
        if let Some(path) = _matches.get_one::<String>("path") {
            let ldf = if _matches.get_flag("text") {
                io::load_csv_from_stdin()
            } else {
                io::read_ipc()
            };
            io::write_parquet(ldf, path.to_string());
        } else {
            eprintln!("Could now write to parquet");
        }
    } else if let Some(_matches) = matches.subcommand_matches("schema") {
        if _matches.get_flag("summary") {
            schema::print_schema(io::read_ipc());
        } else if _matches.get_flag("postgresql") {
            let name = _matches
                .get_one::<String>("name")
                .expect("Please provide a table name");
            let strlen: u32 = match _matches.get_one::<String>("strlen") {
                Some(strlen) => strlen.parse::<u32>().unwrap(),
                None => 128,
            };
            schema::print_create(io::read_ipc(), name.as_str(), strlen);
        }
    } else {
        println!("No command provided. Please execute dr --help")
    }
}
