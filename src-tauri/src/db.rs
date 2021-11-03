use crate::throw;
use log;
use sqlx::migrate::MigrateDatabase;
use sqlx::{ConnectOptions, Sqlite, SqlitePool};

pub async fn init(path: &str) -> Result<SqlitePool, String> {
  let exists = match Sqlite::database_exists(&path).await {
    Ok(exists) => exists,
    Err(e) => throw!("Could not check if database exists: {}", e),
  };
  if !exists {
    match Sqlite::create_database(&path).await {
      Ok(_) => {}
      Err(e) => throw!("Could not create database: {}", e),
    }
  }
  println!("Exists: {}", exists);

  let mut connect_options = sqlx::sqlite::SqliteConnectOptions::new().filename(&path);
  connect_options.log_statements(log::LevelFilter::Info);
  let pool = match SqlitePool::connect_with(connect_options).await {
    Ok(pool) => pool,
    Err(e) => throw!("Could not create database: {}", e),
  };

  match sqlx::migrate!("./migrations").run(&pool).await {
    Ok(_) => {}
    Err(e) => throw!("Could not run database migrations: {}", e),
  };

  Ok(pool)
}
