use clap::{arg, ArgAction, Command};

// Generate command line options for the csv command
pub fn gen_csv_command() -> Command {
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
        )
}

// Generate command line options for the schema command
pub fn gen_schema_command() -> Command {
    Command::new("schema")
        .about("Several table schema related utilities")
        .arg(
            arg!(-i --stdin ... "Read from stdin")
                .required(false)
                .action(ArgAction::SetTrue),
        )
        .arg(arg!(-d --delimiter <String> "Column delimiter. Assume ,").required(false))
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
        )
}

// Generate command line options for the sql command
pub fn gen_sql_command() -> Command {
    Command::new("sql")
        .about("Runs a sql statement on the file")
        .arg(arg!(-d --delimiter <String> "Column delimiter. Assume ,").required(false))
        .arg(arg!([statement] "SQL statement"))
        .arg(
            arg!(-t --text ... "Input text instead of binary")
                .required(false)
                .action(ArgAction::SetTrue),
        )
        .arg(arg!(-d --delimiter <String> "Column delimiter").required(false))
}

// Generate command line options for the rpq command
pub fn gen_rpq_command() -> Command {
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
        .arg(arg!(-P --parquet <String> "Write the result as a parquet file").required(false))
        .arg(
            arg!(-a --head ... "Print the header of the table")
                .required(false)
                .action(ArgAction::SetTrue),
        )
}

// Generate command line options for the wpq command
pub fn gen_wpq_command() -> Command {
    Command::new("wpq")
        .about("Write to a paquet file")
        .arg(arg!(-d --delimiter <String> "Column delimiter. Assume ,").required(false))
        .arg(
            arg!(-t --text ... "Input text instead of binary")
                .required(false)
                .action(ArgAction::SetTrue),
        )
        .arg(arg!([path] "Path to the new parquet file"))
}

// Generate command line options for the print command
pub fn gen_print_command() -> Command {
    Command::new("print")
        .about("Pretty prints the table")
        .arg(arg!(-d --delimiter <String> "Column delimiter. Assume ,").required(false))
        .arg(
            arg!(-t --text ... "Inputs csv instead of binary")
                .required(false)
                .action(ArgAction::SetTrue),
        )
}
