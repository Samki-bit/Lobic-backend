use crate::{
	core::app_state::AppState,
	routes::{
		auth::{login::login, logout::logout, signup::signup, verify::verify},
		music::{
			get_cover_image::get_cover_image, get_music::get_music, increment_times_played::incr_times_played,
			save_music::save_music, search_music::search_music, send_music::send_music,
		},
		playlist::{
			add_song_to_playlist::add_song_to_playlist, create_new_playlist::create_playlist,
			get_playlist_music::get_playlist_music, get_users_playlists::get_users_playlists,
		},
		socket::websocket_handler,
		users::{
			add_friend::add_friend, get_user::get_user, get_user_pfp::get_user_pfp, remove_friend::remove_friend,
			update_pfp::update_pfp,
			search_user::search_user,
		},
	},
};
use axum::{
	routing::{get, post},
	Router,
};

pub fn configure_routes(app_state: AppState) -> Router {
	Router::new()
		.route("/", get(index))
		.route("/get_user", get(get_user))
		.route("/signup", post(signup))
		.route("/login", post(login))
		.route("/logout", post(logout))
		.route("/verify", get(verify))
		.route("/music/:music_id", get(send_music))
		.route("/image/:filename", get(get_cover_image))
		.route("/save_music", post(save_music))
		.route("/get_music", get(get_music))
		.route("/search", get(search_music))
		.route("/playlist/new", post(create_playlist))
		.route("/playlist/add_song", post(add_song_to_playlist))
		.route("/playlist/get_by_uuid", get(get_playlist_music))
		.route("/playlist/get_users_playlists", get(get_users_playlists))
		.route("/music/incr_times_played/:music_uuid", post(incr_times_played))
		.route("/user/update_pfp", post(update_pfp))
		.route("/user/get_pfp/:filename", get(get_user_pfp))
		.route("/add_friend", post(add_friend))
		.route("/remove_friend", post(remove_friend))
		.route("/search_user", get(search_user))
		.route("/ws", get(websocket_handler))
		.with_state(app_state)
}

async fn index() -> String {
	"Hello from Lobic backend".to_string()
}
