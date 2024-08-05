use sqlx::Pool;
use sqlx::postgres::PgPoolOptions;

pub(crate) async fn connect() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("database-1.cxq4mekum4m8.eu-north-1.rds.amazonaws.com:3306")
        .await?;

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    println!("Got: {:?}", row.0);

    Ok(())
}