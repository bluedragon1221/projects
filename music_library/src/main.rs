use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
enum Era {
    Baroque,
    Classical,
    Romantic,
    Impressionist,
    Modern
}

#[derive(Debug, Serialize, Deserialize)]
struct Song {
    path: String,
    opus: Option<i32>,
    number: Option<i32>,
    name: Option<String>,
    composer: String,
    performer: String,
    era: Option<Era>
}

#[derive(Debug, Serialize, Deserialize)]
struct AppData {
    songs: Vec<Song>
}

fn main() {
    let all_songs_file = fs::read_to_string("songs.toml").unwrap_or_default();
    let all_songs: AppData = toml::de::from_str::<AppData>(&all_songs_file).unwrap();

    println!("{:#?}", all_songs)
}
