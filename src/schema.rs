use polars_lazy::prelude::*;
use sea_query::table::ColumnType;
use sea_query::*;

pub fn print_schema(ldf: LazyFrame) {
    let schema = ldf.schema().expect("Could not retreive schema");
    for f in schema.iter_fields() {
        let mut unnamed_cols_counter = 0;
        let d = f.data_type().to_string();
        let n = if f.name.is_empty() {
            unnamed_cols_counter += 1;
            format!("Column{}", unnamed_cols_counter)
        } else {
            f.name
        };

        println!("{n} ({d})");
    }
}

pub fn print_create(ldf: LazyFrame, table_name: &str, default_strlen: u32) {
    let schema = ldf.schema().expect("Could not retreive schema");
    // Create empty table
    let mut statements = vec![Table::create()
        .table(Alias::new(table_name))
        .if_not_exists()
        .to_string(PostgresQueryBuilder)];

    // Alter table adding fields one by one
    let mut unnamed_cols_counter = 0;
    for f in schema.iter_fields() {
        let dtype = match f.data_type().to_string().as_str() {
            "i64" => ColumnType::Integer,
            "f64" => ColumnType::Float,
            "str" => ColumnType::String(Some(default_strlen)),
            "bool" => ColumnType::Boolean,
            &_ => todo!("Datatype {} not supported", f.data_type().to_string()),
        };

        let name = if f.name.is_empty() {
            unnamed_cols_counter += 1;
            format!("Column{}", unnamed_cols_counter)
        } else {
            f.name
        };

        let table = Table::alter()
            .table(Alias::new(table_name))
            .add_column(&mut ColumnDef::new_with_type(Alias::new(&name), dtype))
            .to_owned();
        statements.push(table.to_string(PostgresQueryBuilder));
    }

    // Finallyls print all statements
    for statement in statements {
        println!("{};", statement);
    }
}
