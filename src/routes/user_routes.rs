use std::fs::File;
use std::io::Write;
use axum::extract::Multipart;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::post;
pub fn user_routes() -> Router {
    let router = Router::new()
        .route("/api/user/post", post(upload_image_post));
    return router
}

// Insert multipart in function arguments
pub async fn upload_image_post(mut multipart: Multipart) -> impl IntoResponse {
    // Iterate over field in form-data request
    while let Some(field) = multipart.next_field().await.unwrap() {
        // Get the name = key of iterable field.
        let field_name = field.name().unwrap().to_string();
        // If this name = key == image then extract bytes.
        if field_name == "image" {
            // Extract bytes from field
            let data = field.bytes().await.unwrap();
            // Create an empty file in the filesystem
            let mut file = File::create("./public/test.jpg").unwrap();
            // Convert bytes to &[u8] slice and write it in the file.
            file.write(data.iter().as_slice()).unwrap();
        }
    }
}