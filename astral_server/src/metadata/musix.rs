use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Url;
use serde_json::Value;
use crate::Res;

/// Sends a request to MusixMatch api
pub async fn musix_request(
    title: &str, artist: &str,
    album: &Option<String>, usertoken: &Option<String>
) -> Res<Value> {
    const BASE_URL: &str = "https://apic-desktop.musixmatch.com/ws/1.1/macro.subtitles.get?format=json&namespace=lyrics_richsynched&subtitle_format=mxm&app_id=web-desktop-app-v1.0";

    let uri = Url::parse_with_params(BASE_URL, [
        ("q_album", album.as_ref().unwrap_or(&String::new()).as_str()),
        ("q_artist", artist),
        ("q_artists", artist),
        ("q_track", title),
        ("track_spotify_id", &String::new()),
        ("q_duration", &String::new()),
        ("f_subtitle_length", &String::new()),
        ("usertoken", usertoken.as_ref().unwrap_or(&"211117e30ad03b818a41fd7d38b8191cadf33e797bef89ae809017".to_string()).as_str())
    ]).unwrap();

    let client = reqwest::Client::new();
    let json = client.get(uri)
        .headers(HeaderMap::from_iter([
            (HeaderName::from_static("authority"), HeaderValue::from_static("apic-desktop.musixmatch.com")),
            (HeaderName::from_static("cookie"), HeaderValue::from_static("x-mmm-token-guid="))
        ]))
        .send().await?
        .json::<Value>().await?;

    Ok(json["message"]["body"]["macro_calls"].clone())
}
