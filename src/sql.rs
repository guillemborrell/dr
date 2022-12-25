use polars::sql::SQLContext;
use polars_lazy::frame::LazyFrame;

pub fn execute(ldf: LazyFrame, statement: &String) -> LazyFrame {
    let mut context = SQLContext::try_new().expect("Could not create context");
    context.register("this", ldf);

    match context.execute(statement) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Query execution error {e}");
            LazyFrame::default()
        }
    }
}
