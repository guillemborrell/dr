use polars_sql::SQLContext;
use polars_lazy::prelude::LazyFrame;

pub fn execute(ldf: LazyFrame, statement: &String) -> LazyFrame {
    let mut context = SQLContext::try_new().expect("Could not create context");
    context.register("this", ldf);
    context
        .execute(statement)
        .expect("Could not execute statement")
}
