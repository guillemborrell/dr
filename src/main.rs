mod commands;
mod handlers;
mod io;
mod schema;
mod sql;
use clap::command;

fn main() {
    // Commands definition
    let matches = command!()
        .author("Guillem Borrell")
        .version(env!("CARGO_PKG_VERSION"))
        .about("dr is a handy command line tool to handle csv an parquet files")
        .long_about(
            "dr is a handy command line tool to handle csv and parquet files.
It is designed to integrate nicely with other command line tools
like cat, sed, awk and database clients cli. You can find more
information an a short tutorial https://git.guillemborrell.es/guillem/dr
            ",
        )
        .subcommand(commands::gen_csv_command())
        .subcommand(commands::gen_schema_command())
        .subcommand(commands::gen_sql_command())
        .subcommand(commands::gen_print_command())
        .subcommand(commands::gen_rpq_command())
        .subcommand(commands::gen_wpq_command())
        .get_matches();

    // Send the flow to the corresponding handler
    if let Some(sub_matches) = matches.subcommand_matches("csv") {
        handlers::handle_csv(sub_matches);
    } else if let Some(sub_matches) = matches.subcommand_matches("sql") {
        handlers::handle_sql(sub_matches);
    } else if let Some(sub_matches) = matches.subcommand_matches("print") {
        handlers::handle_print(sub_matches);
    } else if let Some(sub_matches) = matches.subcommand_matches("rpq") {
        handlers::handle_rpq(sub_matches);
    } else if let Some(sub_matches) = matches.subcommand_matches("wpq") {
        handlers::handle_wpq(sub_matches);
    } else if let Some(sub_matches) = matches.subcommand_matches("schema") {
        handlers::handle_schema(sub_matches);
    } else {
        println!("No command provided. Please execute dr --help")
    }
}
