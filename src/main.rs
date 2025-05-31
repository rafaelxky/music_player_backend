mod controllers;

use actix_web::{App, HttpServer};
use actix_cors::Cors;
use controllers::api_controller::*;

// Main function to start the Actix web server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive()) // Enable CORS for all routes
            .service(index)
            .service(fetch_playlists)
            .service(get_track)
            .service(get_playlist)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}