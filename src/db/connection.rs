use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;
pub async fn connect_db() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL is not set")).await?;
    let sql= r#"
    CREATE TABLE IF NOT EXISTS users (
        id SERIAL PRIMARY KEY,
        name VARCHAR(100) NOT NULL,
        email VARCHAR(100) UNIQUE NOT NULL
    );
    "#;
    sqlx::query(sql).execute(&pool).await?;
    println!("Table created successfully.");
    Ok(())
}
