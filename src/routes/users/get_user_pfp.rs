// get user pfp from inside the /storage/users_pfp/<<uuid.png>> by accepting the user_uuid
use crate::config::USER_PFP_STORAGE;

use axum::{
	body::Body,
	extract::Path,
	http::{
		header::{self},
		StatusCode,
	},
	response::{IntoResponse, Response},
};
use std::path::PathBuf;
use tokio::{fs::File, io::AsyncReadExt};

pub async fn get_user_pfp(Path(filename): Path<String>) -> impl IntoResponse {
	// Construct the path to the cover image
	let mut path = PathBuf::from(USER_PFP_STORAGE);
	path.push(&filename);

	// Open the file
	let mut file = match File::open(&path).await {
		Ok(file) => file,
		Err(_) => {
			return (StatusCode::NOT_FOUND, "Image not found").into_response();
		}
	};

	// Read the file into a byte vector
	let mut file_bytes = Vec::new();
	if let Err(_) = file.read_to_end(&mut file_bytes).await {
		return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read image file").into_response();
	}

	let mime_type = match path.extension().and_then(|ext| ext.to_str()) {
		Some("jpg") | Some("jpeg") => "image/jpeg",
		Some("png") => "image/png",
		Some("gif") => "image/gif",
		Some("webp") => "image/webp",
		_ => "application/octet-stream",
	};

	Response::builder()
		.status(StatusCode::OK)
		.header(header::CONTENT_TYPE, mime_type)
		.body(Body::from(file_bytes))
		.unwrap()
}
