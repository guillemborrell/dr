use crate::io::dump_csv_to_stdout;
use crate::io::load_csv_from_stdin;
use polars_lazy::frame::IntoLazy;
use polars_sql::SQLContext;

pub fn execute(statement: &String) {
    if let Ok(mut context) = SQLContext::try_new() {
        let df = load_csv_from_stdin();
        context.register("this", df.lazy());
        if let Ok(res) = context.execute(statement) {
            if let Ok(mut res) = res.collect() {
                dump_csv_to_stdout(&mut res);
            };
        };
        if let Err(e) = context.execute(statement) {
            eprintln!("Query execution error {e}")
        };
    };
}
