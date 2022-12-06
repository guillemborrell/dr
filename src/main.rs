mod io;
mod sql;
use clap::{arg, command, ArgAction, Command};

fn main() {
    let matches = command!()
        .subcommand(
            Command::new("csv")
                .about("Read csv, output arrow stream")
                .arg(arg!([path] "Path to CSV file"))
                .arg(arg!(-d --delimiter <String> "Column delimiter. Assume ,").required(false))
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
                    arg!(-t --text ... "Output text instead of binary")
                        .required(false)
                        .action(ArgAction::SetTrue),
                )
                .arg(arg!(-P --text <String> "Write the result as a parquet file").required(false))
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
                    arg!(-t --text ... "Output text instead of binary")
                        .required(false)
                        .action(ArgAction::SetTrue),
                )
                .arg(arg!([path] "Path to the new parquet file")),
        )
        .get_matches();

    if let Some(_matches) = matches.subcommand_matches("csv") {
        if let Some(path) = _matches.get_one::<String>("path") {
            let mut ldf = io::read_csv(path.to_string());
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
                    io::dump_csv_to_stdout(&mut ldf.collect().expect("Could not collect"));
                } else {
                    if let Some(path) = _matches.get_one::<String>("parquet") {
                        io::write_parquet(
                            ldf.collect().expect("Could not collect"),
                            path.to_string(),
                            "lz4raw".to_string(),
                            true,
                            Some(0),
                        );
                    } else {
                        io::write_ipc(ldf);
                    }
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
        if let Some(path) = _matches.get_one::<String>("path") {
            let mut ldf = io::read_parquet(path.to_string());
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
                    io::dump_csv_to_stdout(&mut ldf.collect().expect("Could not collect"));
                } else {
                    if let Some(path) = _matches.get_one::<String>("parquet") {
                        io::write_parquet(
                            ldf.collect().expect("Could not collect"),
                            path.to_string(),
                            "lz4raw".to_string(),
                            true,
                            Some(0),
                        );
                    } else {
                        io::write_ipc(ldf);
                    }
                }
            }
        } else {
            eprintln!("File not found")
        }
    } else if let Some(_matches) = matches.subcommand_matches("wpq") {
        if let Some(path) = _matches.get_one::<String>("path") {
            let df = if _matches.get_flag("text") {
                io::load_csv_from_stdin()
            } else {
                io::read_ipc()
            };
            io::write_parquet(
                df.collect().expect("Could not collect"),
                path.to_string(),
                "lz4raw".to_string(),
                true,
                Some(0),
            );
        } else {
            eprintln!("Could now write to parquet");
        }
    } else {
        println!("No command provided. Please execute dr --help")
    }
}
