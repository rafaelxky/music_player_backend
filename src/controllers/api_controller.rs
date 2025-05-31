use actix_web::{get, web, Responder, HttpResponse};
use serde::{Serialize, Deserialize};

// Helper to generate demo playlists
fn get_playlists() -> Vec<Playlist> {
    vec![
        Playlist {
            name: String::from("My Favorite Songs"),
            tracks: vec![
                Track {
                    title: String::from("Song 1"),
                    artist: String::from("Artist A"),
                    duration: 240,
                    location: String::from("/music/song1.mp3"),
                },
                Track {
                    title: String::from("Song 2"),
                    artist: String::from("Artist B"),
                    duration: 180,
                    location: String::from("/music/song2.mp3"),
                },
            ],
        }
    ]
}

// Test endpoint
#[get("/")]
pub async fn index() -> impl Responder {
    "Welcome to the Music Player Backend!"
}

// Returns a list of playlists and their tracks
#[get("/playlist")]
pub async fn fetch_playlists() -> impl Responder {
    let playlists = get_playlists();
    HttpResponse::Ok().json(playlists)
}

// Returns a playlist by index
#[get("/playlist/{index_pllst}")]
pub async fn get_playlist(index_pllst: web::Path<usize>) -> impl Responder {
    let playlists = get_playlists();
    if let Some(playlist) = playlists.get(*index_pllst) {
        HttpResponse::Ok().json(playlist)
    } else {
        HttpResponse::NotFound().body("Playlist not found")
    }
}

// Returns a track by index 
#[get("/playlist/{index_pllst}/track/{index_trck}")]
pub async fn get_track(path: web::Path<(usize, usize)>) -> impl Responder {
    let (index_pllst, index_trck) = path.into_inner();
    let playlists = get_playlists();
    if let Some(track) = playlists.get(index_pllst).and_then(|p| p.tracks.get(index_trck)) {
        HttpResponse::Ok().json(track)
    } else {
        HttpResponse::NotFound().body("Track not found")
    }
}

// Define the data structures for playlists and tracks
#[derive(Serialize, Deserialize)]
pub struct Playlist {
    pub name: String,
    pub tracks: Vec<Track>,
}

#[derive(Serialize, Deserialize)]
pub struct Track {
    pub title: String,
    pub artist: String,
    pub duration: u32,
    pub location: String,
}

