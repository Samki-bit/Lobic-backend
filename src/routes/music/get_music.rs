use axum::{
	extract::{Query, State},
	http::{header, StatusCode},
	response::Response,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;

use crate::config::{COVER_IMG_STORAGE, MUSIC_STORAGE};
use crate::{core::app_state::AppState, lobic_db::models::Music};

#[derive(Debug, Serialize, Deserialize)]
pub struct MusicResponse {
	pub id: String,
	pub filename: String,
	pub artist: String,
	pub title: String,
	pub album: String,
	pub genre: String,
	pub times_played: i32,
	pub cover_art_path: Option<String>,
}

#[derive(Deserialize)]
pub struct MusicQuery {
	title: Option<String>,
	uuid: Option<String>,
}

pub async fn get_music(State(app_state): State<AppState>, Query(params): Query<MusicQuery>) -> Response<String> {
	// Step 1: Get a database connection
	let mut db_conn = match app_state.db_pool.get() {
		Ok(conn) => conn,
		Err(err) => {
			let msg = format!("Failed to get DB from pool: {err}");
			return Response::builder()
				.status(StatusCode::INTERNAL_SERVER_ERROR)
				.body(msg)
				.unwrap();
		}
	};

	use crate::schema::music::dsl::*;

	let mut query = music.into_boxed();

	// Step 2: Build query based on provided parameters
	match (params.title, params.uuid) {
		(Some(title_val), None) => {
			query = query.filter(title.eq(title_val));
		}
		(None, Some(uuid_val)) => {
			query = query.filter(music_id.eq(uuid_val));
		}
		(None, None) => {
			// No parameters - return all music
		}
		(Some(_), Some(_)) => {
			return Response::builder()
				.status(StatusCode::BAD_REQUEST)
				.body("Please provide either title or uuid, not both".to_string())
				.unwrap();
		}
	}

	// Step 3: Execute the query and handle the results
	let result = query.load::<Music>(&mut db_conn);

	match result {
		Ok(music_entries) => {
			if music_entries.is_empty() {
				return Response::builder()
					.status(StatusCode::NOT_FOUND)
					.body("No music entries found".to_string())
					.unwrap();
			}

			// Step 4: Map the database entries to the response format
			let responses: Vec<MusicResponse> = music_entries
				.into_iter()
				.map(|entry| {
					let cover_art_path = format!("{}/{}.png", COVER_IMG_STORAGE, entry.music_id);
					let has_cover = fs::metadata(&cover_art_path).is_ok();

					MusicResponse {
						id: entry.music_id.clone(),
						filename: format!("{}/{}.mp3", MUSIC_STORAGE, entry.music_id),
						artist: entry.artist,
						title: entry.title,
						album: entry.album,
						genre: entry.genre,
						times_played: entry.times_played,
						cover_art_path: has_cover.then_some(cover_art_path),
					}
				})
				.collect();

			// Step 5: Serialize the response and return it
			match serde_json::to_string(&responses) {
				Ok(json) => Response::builder()
					.status(StatusCode::OK)
					.header(header::CONTENT_TYPE, "application/json")
					.body(json)
					.unwrap(),
				Err(err) => Response::builder()
					.status(StatusCode::INTERNAL_SERVER_ERROR)
					.body(format!("Failed to serialize response: {err}"))
					.unwrap(),
			}
		}
		Err(err) => Response::builder()
			.status(StatusCode::INTERNAL_SERVER_ERROR)
			.body(format!("Database error: {err}"))
			.unwrap(),
	}
}
