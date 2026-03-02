mod handlers;

use std::collections::HashMap;
use axum::routing::{Router, delete, get, post, put};
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::sync::{Arc, Mutex};
use dotenvy::dotenv;
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    dotenv().ok();
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let database_url = env::var("DATABASE_URL").expect("missing DATABASE_URL env");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let state = AppState { pool };

    let api_routes = Router::new()
        .route("/", get(handlers::hello_world))
        .route("/articles", get(handlers::get_articles))
        .route("/cities", get(handlers::get_cities))
        .route("/deals", get(handlers::get_deals))
        .route("/photos", get(handlers::get_photos))
        .route("/shops", get(handlers::get_shops))
        .route("/users", get(handlers::get_users))
        .route("/users", post(handlers::post_users))

/*        .route("/users_conversations/{id}", get(handlers::get_users_conversations))
        .route("/users/{id}", get(handlers::get_user_by_id))
        .route("/users/name={username}", get(handlers::get_user_by_username))
        .route("/send_message", post(handlers::send_message))
        .route("/register_user", post(handlers::register_user))
        .route("/login_user", post(handlers::login_user))
        .route("/create_conversation", post(handlers::create_conversation))

        .route("/get_users_debug", get(handlers::get_users_DEBUG))
        .route("/get_conversations_debug", get(handlers::get_conversations_DEBUG))
        .route("/get_conversationsusers_debug", get(handlers::get_conversationsusers_DEBUG))
        .route("/get_friends_debug", get(handlers::get_friends_DEBUG))
        .route("/get_friendrequests_debug", get(handlers::get_friendrequests_DEBUG))
        .route("/get_messages_debug", get(handlers::get_messages_DEBUG))
*/
        .layer(cors)
        .with_state(state.pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, api_routes)
        .await?;

    Ok(())
}

#[derive(Clone)]
struct AppState {
    pool: PgPool
}
