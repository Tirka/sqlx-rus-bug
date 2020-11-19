
#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "info");
    // std::env::set_var("RUST_LOG", "error"); // uncomment to "fix" panic
    
    // provide valid connection string
    let conn_str = std::env::var("DATABASE_URL")
        .expect("valid `DATABASE_URL` environment variable must be set");
    
    let pool = sqlx::postgres::PgPool::connect(&conn_str).await
        .unwrap();
    
    pretty_env_logger::init();

    let query = "\nSELECT 'главная'"; // bad query
    // let query = "SELECT 'главная'"; // good query
    // let query = "\nSELECT 'main'"; // good query

    let _row = sqlx::query(query)
        .fetch_one(&pool).await
        .unwrap();

    println!("no errors occured!");
}