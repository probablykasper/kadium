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
      let code = error_obj.get("code").map(|v| v.as_i64()).flatten();
      let code_str = code.map(|n| n.to_string()).unwrap_or_default();
      let message = error_obj.get("message").map(|v| v.as_str()).flatten();
      throw!("API error: {} {}", code_str, message.unwrap_or_default());
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

pub mod videos {
  use serde::Deserialize;

  /// Lists the fields we use only. Documentation:
  /// https://developers.google.com/youtube/v3/docs/videos/list#properties
  #[derive(Deserialize, Debug)]
  pub struct Response {
    pub items: Vec<Video>,
  }
  /// Lists the fields we use only. Documentation:
  /// https://developers.google.com/youtube/v3/docs/videos#properties
  #[derive(Deserialize, Debug)]
  #[allow(non_snake_case)]
  pub struct Video {
    pub id: String,
    pub contentDetails: ContentDetails,
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
  /// Lists the fields we use only. Documentation:
  /// https://developers.google.com/youtube/v3/docs/playlistItems#properties
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
    pub videoPublishedAt: String,
    pub videoId: String,
  }
}
