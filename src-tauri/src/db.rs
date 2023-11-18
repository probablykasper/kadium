use crate::api::playlist_items;
use crate::data::{AppPaths, DataState};
use crate::throw;
use serde::{Deserialize, Serialize};
use specta::Type;
use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::{SqliteConnectOptions, SqliteRow};
use sqlx::{Row, Sqlite, SqlitePool};
use tauri::command;

pub async fn init(app_paths: &AppPaths) -> Result<SqlitePool, String> {
  let exists = match Sqlite::database_exists(&app_paths.db).await {
    Ok(exists) => exists,
    Err(e) => throw!("Could not check if database exists: {}", e),
  };
  if !exists {
    if let Err(e) = std::fs::create_dir_all(&app_paths.app_dir) {
      throw!("Error creating parent folder: {}", e.to_string());
    }
    match Sqlite::create_database(&app_paths.db).await {
      Ok(_) => {}
      Err(e) => throw!("Could not create database: {}", e),
    }
  }

  let connect_options = SqliteConnectOptions::new().filename(&app_paths.db);
  let pool = match SqlitePool::connect_with(connect_options).await {
    Ok(pool) => pool,
    Err(e) => throw!("Could not open database: {}", e),
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

#[derive(Debug, Serialize, Type)]
#[allow(non_snake_case)]
pub struct Video {
  pub id: String,
  pub title: String,
  pub description: String,
  #[specta(type = i32)] // tauri bigint fix
  pub publishTimeMs: i64,
  /// SQLite does not support unsigned integers
  #[specta(type = i32)] // tauri bigint fix
  pub durationMs: i64,
  pub thumbnailStandard: bool,
  pub thumbnailMaxres: bool,
  pub channelId: String,
  pub channelName: String,
  pub unread: bool,
  pub archived: bool,
}
impl sqlx::FromRow<'_, SqliteRow> for Video {
  fn from_row(row: &SqliteRow) -> sqlx::Result<Self> {
    Ok(Video {
      id: row.try_get("id")?,
      title: row.try_get("title")?,
      description: row.try_get("description")?,
      publishTimeMs: row.try_get("publishTimeMs")?,
      // SQLite does not support unsigned integers
      durationMs: row.try_get("durationMs")?,
      thumbnailStandard: row.try_get("thumbnailStandard")?,
      thumbnailMaxres: row.try_get("thumbnailMaxres")?,
      channelId: row.try_get("channelId")?,
      channelName: row.try_get("channelName")?,
      unread: row.try_get("unread")?,
      archived: row.try_get("archived")?,
    })
  }
}

pub async fn insert_video(video: &Video, pool: &SqlitePool) -> Result<(), String> {
  let query_str = 
    "INSERT INTO videos (id,title,description,publishTimeMs,durationMs,thumbnailStandard,thumbnailMaxres,channelId,channelName) \
    VALUES (?,?,?,?,?,?,?,?,?)";
  let query = sqlx::query(query_str)
    .bind(&video.id)
    .bind(&video.title)
    .bind(&video.description)
    .bind(video.publishTimeMs)
    .bind(video.durationMs)
    .bind(video.thumbnailStandard)
    .bind(video.thumbnailMaxres)
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

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct Options {
  show_all: bool,
  show_archived: bool,
  channel_filter: String,
  tag: Option<String>,
  limit: u16,
}
#[derive(Debug, Serialize, Deserialize, Type)]
#[allow(non_snake_case)]
pub struct After {
  #[specta(type = i32)] // tauri bigint fix
  publishTimeMs: i64,
  id: String,
}

#[command]
#[specta::specta]
pub async fn get_videos(
  options: Options,
  after: Option<After>,
  data: DataState<'_>,
) -> Result<Vec<Video>, String> {
  let data = data.0.lock().await;
  let mut selects: Vec<&str> = vec!["*"];
  let mut wheres: Vec<&str> = Vec::new();
  let mut bindings: Vec<&str> = Vec::new();
  if options.channel_filter != "" {
    selects.push("INSTR(channelName, ?) channelFilter");
    wheres.push("channelFilter > 0");
    bindings.push(&options.channel_filter);
  }
  if !options.show_all {
    if options.show_archived {
      wheres.push("archived = 1");
    } else {
      wheres.push("archived = 0");
    }
  }
  let after_publish_time_ms;
  if let Some(after) = &after {
    wheres.push("(publishTimeMs,id) < (?,?)");
    after_publish_time_ms = after.publishTimeMs.to_string();
    bindings.push(&after_publish_time_ms);
    bindings.push(&after.id);
  }

  let q;
  if let Some(tag) = &options.tag {
    let mut question_marks: Vec<&str> = Vec::new();
    for channel in &data.settings_ref().channels {
      if channel.tags.contains(tag) {
        bindings.push(&channel.id);
        question_marks.push("?");
      }
    }
    q = format!("channelId IN ({})", question_marks.join(","));
    wheres.push(&q);
  }

  let mut query_str = "SELECT ".to_owned() + &selects.join(",");
  query_str.push_str(" FROM videos");
  if !wheres.is_empty() {
    query_str.push_str(" WHERE ");
    query_str.push_str(&wheres.join(" AND "));
  }
  query_str.push_str(" ORDER BY publishTimeMs DESC, id DESC");
  query_str.push_str(&format!(" LIMIT {}", options.limit));

  let mut query = sqlx::query_as(&query_str);
  for binding in bindings {
    query = query.bind(binding);
  }
  let videos: Vec<Video> = match query.fetch_all(&data.db_pool).await {
    Ok(videos) => videos,
    Err(e) => throw!("Error getting videos: {}", e),
  };
  Ok(videos)
}

async fn set_archived(pool: &SqlitePool, id: &str, value: bool) -> Result<(), String> {
  let query = sqlx::query("UPDATE videos SET archived = ? WHERE id = ?")
    .bind(value)
    .bind(id);
  let rows_affected = match query.execute(pool).await {
    Ok(result_rows) => result_rows.rows_affected(),
    Err(e) => throw!("{}", e),
  };
  if rows_affected != 1 {
    throw!("{} rows affected", rows_affected);
  }
  Ok(())
}

#[command]
#[specta::specta]
pub async fn archive(id: String, data: DataState<'_>) -> Result<(), String> {
  let data = data.0.lock().await;
  match set_archived(&data.db_pool, &id, true).await {
    Ok(()) => Ok(()),
    Err(e) => throw!("Error archiving video: {}", e),
  }
}

#[command]
#[specta::specta]
pub async fn unarchive(id: String, data: DataState<'_>) -> Result<(), String> {
  let data = data.0.lock().await;
  match set_archived(&data.db_pool, &id, false).await {
    Ok(()) => Ok(()),
    Err(e) => throw!("Error unarchiving video: {}", e),
  }
}
