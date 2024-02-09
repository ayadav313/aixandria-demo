use actix_files as fs;
use actix_web::{App, HttpServer};
use actix_rt::main; // Import the `main` attribute from `actix_rt`

#[main] // Use the `main` attribute from `actix_rt`
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/", "./static").index_file("index.html"))
            .service(fs::Files::new("/classroom", "./static").index_file("classroom.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
