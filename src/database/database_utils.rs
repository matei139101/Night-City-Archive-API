use sea_orm::{Database, DatabaseConnection};

pub async fn make_database_connection() -> DatabaseConnection {
    let _ = dotenvy::dotenv();
    let database_host = std::env::var("DB_HOST").expect("DB_HOST not set");
    let database_port = std::env::var("DB_PORT").expect("DB_PORT not set");
    let database_user = std::env::var("DB_USER").expect("DB_USER not set");
    let database_password = std::env::var("DB_PASS").expect("DB_PASS not set");
    let database_name = std::env::var("DB_NAME").expect("DB_NAME not set");

    let database_url =
        format!("mysql://{database_user}:{database_password}@{database_host}:{database_port}/{database_name}?ssl-mode=DISABLED");

    let db: DatabaseConnection = Database::connect(&database_url)
        .await
        .expect("Couldn't create a database connection");

    println!("Connected to Night City Archive database");

    db
}