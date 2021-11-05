use crate::api::playlist_items;
use crate::throw;
use log;
use serde::Serialize;
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
    query = query.bind(&video.contentDetails.videoId);
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

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct Video {
  pub id: String,
  pub title: String,
  pub description: String,
  pub publishTimeMs: i64,
  /// SQLite does not support unsigned integers
  pub durationMs: i64,
  pub thumbnailStandard: bool,
  pub thumbnailMaxres: bool,
  pub channelId: String,
  pub channelName: String,
  pub unread: bool,
}
impl sqlx::FromRow<'_, sqlx::sqlite::SqliteRow> for Video {
  fn from_row(row: &sqlx::sqlite::SqliteRow) -> sqlx::Result<Self> {
    Ok(Video {
      id: row.try_get("id")?,
      title: row.try_get("title")?,
      description: row.try_get("description")?,
      publishTimeMs: row.try_get("publishTimeMs")?,
      /// SQLite does not support unsigned integers
      durationMs: row.try_get("durationMs")?,
      thumbnailStandard: row.try_get("thumbnailStandard")?,
      thumbnailMaxres: row.try_get("thumbnailMaxres")?,
      channelId: row.try_get("channelId")?,
      channelName: row.try_get("channelName")?,
      unread: row.try_get("unread")?,
    })
  }
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
    .bind(&video.publishTimeMs)
    .bind(&video.durationMs)
    .bind(&video.thumbnailStandard)
    .bind(&video.thumbnailMaxres)
    .bind(&video.channelId)
    .bind(&video.channelName);
  let rows_affected = match query.execute(pool).await {
    Ok(result_rows) => result_rows.rows_affected(),
    Err(e) => throw!("Error saving video: {}", e),
  };
  if rows_affected != 1 {
    throw!("Error saving video: {} rows affected", rows_affected);
  }
  Ok(())
}

pub async fn get_videos(pool: &SqlitePool) -> Result<Vec<Video>, String> {
  let query = sqlx::query_as("SELECT * FROM videos");
  let videos: Vec<Video> = match query.fetch_all(pool).await {
    Ok(videos) => videos,
    Err(e) => throw!("Error getting videos: {}", e),
  };
  Ok(videos)
}
