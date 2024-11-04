// use axum::{routing::get, Router};
// use std::net::SocketAddr;

// #[tokio::main]
// async fn main() {
//     // Building our application with a single Route
//     let app = Router::new().route("/", get(handler));

//     let port = std::env::var("PORT").unwrap_or("80".to_string());
//     let port = port.parse::<u16>().unwrap_or_else(|_| {
//         eprintln!("Invalid port number: {}", port);
//         std::process::exit(1);
//     });
//     // Run the server with hyper on http://127.0.0.1:3000
//     let addr = SocketAddr::from(([127, 0, 0, 1], port));
//     eprintln!("Listening on http://{}", addr);
//     axum::Server::bind(&addr)
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }

// async fn handler() -> &'static str {
//     "Hello, Axum ❤️ WASIX!"
// }

// main.rs
use axum::{extract::Multipart, http::StatusCode, routing::post, Router};
use std::net::SocketAddr;
use std::process::Command;

#[tokio::main]
async fn main() {
    // Define a simple router with a single POST route
    let app = Router::new().route("/codecs", post(print_codecs));

    // Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Handler function to extract and print codecs
async fn print_codecs(mut multipart: Multipart) -> (StatusCode, String) {
    // Process the file in the request
    while let Some(field) = multipart.next_field().await.unwrap() {
        if let Some(filename) = field.file_name() {
            // Save the file to a temporary path
            let temp_path = format!("/tmp/{}", filename);
            tokio::fs::write(&temp_path, field.bytes().await.unwrap())
                .await
                .unwrap();

            // Run FFmpeg to get codec information
            let output = Command::new("/bin/ffmpeg")
                .arg("-i")
                .arg(&temp_path)
                .arg("-hide_banner") // Hide unnecessary FFmpeg output
                .output()
                .expect("Failed to execute FFmpeg");

            // Clean up the uploaded file
            tokio::fs::remove_file(&temp_path).await.unwrap();

            // Extract codec information and return as response
            let output_text = String::from_utf8_lossy(&output.stderr);
            return (StatusCode::OK, output_text.into_owned());
        }
    }
    (StatusCode::BAD_REQUEST, "No file provided".to_string())
}
