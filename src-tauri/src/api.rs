use crate::throw;
use serde::de::DeserializeOwned;

pub async fn yt_request<T: DeserializeOwned>(url: &str, key: &str) -> Result<T, String> {
  let client = reqwest::Client::new();
  let json: serde_json::Value = client
    .get(url)
    .header("X-Goog-Api-Key", key)
    .send()
    .await
    .map_err(|e| format!("API request failed: {}", e))?
    .json()
    .await
    .map_err(|e| format!("API response was not JSON: {}", e))?;

  match json.get("error") {
    Some(error_obj) => {
      let code = error_obj.get("code").and_then(|v| v.as_i64());
      let code_str = code.map(|n| n.to_string()).unwrap_or_default();
      let message = error_obj.get("message").and_then(|v| v.as_str());
      println!("{:?}", json);
      throw!("{} {}", code_str, message.unwrap_or_default());
    }
    _ => {}
  }
  match serde_json::from_value::<T>(json) {
    Ok(v) => Ok(v),
    Err(e) => {
      throw!("Unexpected API response: {}", e);
    }
  }
}

pub async fn channel_id_from_video_id(id: &str, key: &str) -> Result<String, String> {
  let url =
    "https://youtube.googleapis.com/youtube/v3/videos".to_string() + "?part=snippet" + "&id=" + id;
  let videos = yt_request::<videos::Response>(&url, key)
    .await
    .map_err(|e| format!("Failed to get video: {}", e))?;
  if let Some(video) = videos.items.first() {
    return Ok(video.snippet.channelId.clone());
  }
  Err("No video returned".to_string())
}

pub async fn channel_id_from_username(username: &str, key: &str) -> Result<String, String> {
  let url = "https://youtube.googleapis.com/youtube/v3/channels".to_string()
    + "?part=contentDetails,id,snippet"
    + "&forUsername="
    + username;
  let channels = yt_request::<channels::Response>(&url, key)
    .await
    .map_err(|e| format!("Failed to get video: {}", e))?;
  if channels.items.len() > 1 {
    return Err("YouTube username search returned in multiple channels".to_string());
  }
  if let Some(channel) = channels.items.first() {
    return Ok(channel.id.clone());
  }
  Err("No video returned".to_string())
}

pub mod channels {
  use serde::Deserialize;

  /// Lists the fields we use only. Documentation:
  /// https://developers.google.com/youtube/v3/docs/channels/list#properties
  #[derive(Deserialize, Debug)]
  pub struct Response {
    pub items: Vec<Channel>,
  }
  #[derive(Deserialize, Debug)]
  #[allow(non_snake_case)]
  pub struct Channel {
    pub id: String,
    pub contentDetails: ContentDetails,
    pub snippet: Snippet,
  }

  #[derive(Deserialize, Debug)]
  #[allow(non_snake_case)]
  pub struct ContentDetails {
    pub relatedPlaylists: RelatedPlaylists,
  }
  #[derive(Deserialize, Debug)]
  pub struct RelatedPlaylists {
    pub uploads: String,
  }

  #[derive(Deserialize, Debug)]
  pub struct Snippet {
    pub title: String,
    pub thumbnails: Thumbnails,
  }
  /// default 88x88, medium 240x240, high 800x800
  #[derive(Deserialize, Debug)]
  pub struct Thumbnails {
    pub medium: Thumbnail,
  }
  #[derive(Deserialize, Debug)]
  pub struct Thumbnail {
    pub url: String,
  }
}

pub mod videos {
  use serde::Deserialize;

  /// Lists the fields we use only. Documentation:
  /// https://developers.google.com/youtube/v3/docs/videos/list#properties
  #[derive(Deserialize, Debug)]
  pub struct Response {
    pub items: Vec<Video>,
  }
  #[derive(Deserialize, Debug)]
  #[allow(non_snake_case)]
  pub struct Video {
    pub id: String,
    pub contentDetails: Option<ContentDetails>,
    pub liveStreamingDetails: Option<LiveStreamingDetails>,
    pub snippet: Snippet,
  }
  #[derive(Deserialize, Debug)]
  pub struct ContentDetails {
    pub duration: String,
  }
  #[derive(Deserialize, Debug)]
  #[allow(non_snake_case)]
  pub struct LiveStreamingDetails {
    pub actualStartTime: Option<String>,
  }

  #[derive(Deserialize, Debug)]
  #[allow(non_snake_case)]
  pub struct Snippet {
    pub publishedAt: String,
    pub title: String,
    pub description: String,
    pub thumbnails: Thumbnails,
    pub channelId: String,
    pub channelTitle: String,
  }
  /// default, medium and high always exist:
  /// default 120x90:   https://i.ytimg.com/vi/___ID___/default.jpg
  /// medium 320x180:   https://i.ytimg.com/vi/___ID___/mqdefault.jpg
  /// high 480x360:     https://i.ytimg.com/vi/___ID___/hqdefault.jpg
  /// standard 640x480: https://i.ytimg.com/vi/___ID___/sddefault.jpg
  /// maxres 1280x720:  https://i.ytimg.com/vi/___ID___/maxresdefault.jpg
  #[derive(Deserialize, Debug)]
  pub struct Thumbnails {
    pub standard: Option<Thumbnail>,
    pub maxres: Option<Thumbnail>,
  }
  #[derive(Deserialize, Debug)]
  pub struct Thumbnail {
    pub url: String,
  }
}

pub mod playlist_items {
  use serde::Deserialize;

  /// Lists the fields we use only. Documentation:
  /// https://developers.google.com/youtube/v3/docs/playlistItems/list#properties
  #[derive(Deserialize, Debug)]
  pub struct Response {
    pub items: Vec<Playlist>,
  }
  /// weird date situation:
  ///  `snippet.publishedAt` is when the video was added to the uploads playlist.
  ///  `contentDetails.videoPublishedAt` is when the video was published
  ///  Soemtimes these are a few seconds different, other times an hour
  ///  (like with Monstercat). No idea why.
  ///  Additionally, I tried to compare with what YouTube shows:
  ///  - What YouTube shows: 19:56 (should be up to 1 hour inaccurate)
  ///  - publishedAt: 17:31
  ///  - 18:00
  ///  YouTube only shows "9 hours ago", so you'd expect it to be up to
  ///  an hour off... But it's almost 2 hours off, if not 2.5 hours.
  ///  :/
  #[derive(Deserialize, Debug)]
  #[allow(non_snake_case)]
  pub struct Playlist {
    pub contentDetails: ContentDetails,
  }
  #[derive(Deserialize, Debug)]
  #[allow(non_snake_case)]
  pub struct ContentDetails {
    /// Soemtimes videoPublishedAt is missing when videos are privated, but still included in the channel video list.
    pub videoPublishedAt: Option<String>,
    pub videoId: String,
  }
}
