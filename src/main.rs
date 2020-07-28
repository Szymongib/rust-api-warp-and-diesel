#[macro_use]
extern crate diesel;

mod model;
mod errors;
mod data_access;
mod schema;
mod api;

use std::env;
use warp::{Filter, reject};
use log::{info};

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

type PgPool = Pool<ConnectionManager<PgConnection>>;

fn pg_pool(db_url: &str) -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::new(manager).expect("Postgres connection pool could not be created")
}

use crate::data_access::DBAccessManager;
use crate::errors::{AppError, ErrorType};

fn with_db_access_manager(pool: PgPool) -> impl Filter<Extract = (DBAccessManager,), Error = warp::Rejection> + Clone {
    warp::any()
        .map(move || pool.clone())
        .and_then(|pool: PgPool| async move {  match pool.get() {
            Ok(conn) => Ok(DBAccessManager::new(conn)),
            Err(err) => Err(reject::custom(
                AppError::new(format!("Error getting connection from pool: {}", err.to_string()).as_str(), ErrorType::Internal))
            ),
        }})
}

use serde::de::DeserializeOwned;

fn with_json_body<T: DeserializeOwned + Send>(
) -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL env not set");

    let pg_pool = pg_pool(database_url.as_str());

    let routes = api_filters(pg_pool)
        .recover(errors::handle_rejection);

    info!("Starting server on port 3030...");

    // Start up the server...
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn api_filters(
    pool: PgPool
) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone  {
    warp::path!("api" / "v1" / ..)   // Add path prefix /api/v1 to all our routes
        .and(
            add_book(pool.clone())
                .or(update_status(pool.clone()))
                .or(delete_book(pool.clone()))
                .or(list_books(pool))
        )
}

use crate::api::{AddBook, UpdateStatus};

/// POST /books
fn add_book(
    pool: PgPool
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("books")                    // Match /books path
        .and(warp::post())                  // Match POST method
        .and(with_db_access_manager(pool))  // Add DBAccessManager to params tuple
        .and(with_json_body::<AddBook>())   // Try to deserialize JSON body to AddBook
        .and_then(api::add_book)            // Pass the params touple to the handler function
}

/// GET /books
fn list_books(
    pool: PgPool
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("books")
        .and(warp::get())
        .and(with_db_access_manager(pool))
        .and_then(api::list_books)
}

/// PUT /books/:id
fn update_status(
    pool: PgPool
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("books" / i64 )
        .and(warp::put())
        .and(with_db_access_manager(pool))
        .and(with_json_body::<UpdateStatus>())
        .and_then(api::update_status)
}

/// DELETE /books/:id
fn delete_book(
    pool: PgPool
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("books" / i64 )
        .and(warp::delete())
        .and(with_db_access_manager(pool))
        .and_then(api::delete_book)
}
