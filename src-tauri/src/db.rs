use crate::api::playlist_items;
use crate::throw;
use log;
use sqlx::migrate::MigrateDatabase;
use sqlx::{ConnectOptions, Row, Sqlite, SqlitePool};

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

pub async fn get_ids(
  videos: &Vec<playlist_items::Playlist>,
  pool: &SqlitePool,
) -> Result<Vec<String>, String> {
  // let mut id_placeholders = "\"?\"".to_string();
  let mut id_placeholders = "?".to_string();
  for _n in 0..(videos.len() - 1) {
    // id_placeholders.push_str(",\"?\"");
    id_placeholders.push_str(",?");
  }

  let query_str = format!("SELECT id FROM videos WHERE id IN ({});", id_placeholders);
  let mut query = sqlx::query(&query_str);
  for video in videos {
    query = query.bind(&video.content_details.video_id);
  }
  let rows = match query.fetch_all(pool).await {
    Ok(rows) => rows,
    Err(e) => throw!("Unable to get video IDs: {}", e),
  };
  let mut existing_ids: Vec<String> = Vec::new();
  for row in rows {
    match row.try_get(0) {
      Ok(id) => existing_ids.push(id),
      Err(e) => throw!("Unable to get video ID from database row: {}", e),
    };
  }
  Ok(existing_ids)
}

pub struct Video {
  pub id: String,
  pub title: String,
  pub description: String,
  pub publish_time_ms: i64,
  /// SQLite does not support unsigned integers
  pub duration_ms: i64,
  pub thumbnail_standard: bool,
  pub thumbnail_maxres: bool,
  pub channel_id: String,
  pub channel_name: String,
  pub unread: bool,
}

pub async fn insert_video(video: &Video, pool: &SqlitePool) -> Result<(), String> {
  let query_str = format!(
    "INSERT INTO videos (id,title,description,publishTimeMs,durationMs,thumbnailStandard,thumbnailMaxres,channelId,channelName) \
    VALUES (?,?,?,?,?,?,?,?,?)"
  );
  let query = sqlx::query(&query_str)
    .bind(&video.id)
    .bind(&video.title)
    .bind(&video.description)
    .bind(&video.publish_time_ms)
    .bind(&video.duration_ms)
    .bind(&video.thumbnail_standard)
    .bind(&video.thumbnail_maxres)
    .bind(&video.channel_id)
    .bind(&video.channel_name);
  let rows_affected = match query.execute(pool).await {
    Ok(result_rows) => result_rows.rows_affected(),
    Err(e) => throw!("Error saving video: {}", e),
  };
  if rows_affected != 1 {
    throw!("Error saving video: {} rows affected", rows_affected);
  }
  Ok(())
}
