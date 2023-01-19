mod commands;
mod handlers;
mod io;
mod schema;
mod sql;
use clap::command;

fn main() {
    let matches = command!()
        .subcommand(commands::gen_csv_command())
        .subcommand(commands::gen_schema_command())
        .subcommand(commands::gen_sql_command())
        .subcommand(commands::gen_print_command())
        .subcommand(commands::gen_rpq_command())
        .subcommand(commands::gen_wpq_command())
        .get_matches();
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
