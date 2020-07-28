use serde_derive::{Deserialize, Serialize};
use crate::model::{BookStatus, CreateBookDTO};
use crate::AppError;
use crate::data_access::DBAccessManager;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AddBook {
    pub title: String,
    pub author: String,
    pub status: BookStatus,
}

impl AddBook {
    pub fn to_dto(&self) -> CreateBookDTO {
        CreateBookDTO{
            title: self.title.clone(),
            author: self.author.clone(),
            status: self.status.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateStatus {
    pub status: BookStatus,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct IdResponse {
    pub id: i64,
}

impl IdResponse {
    pub fn new(id: i64) -> IdResponse {
        IdResponse { id }
    }
}

pub async fn add_book(
    db_manager: DBAccessManager,
    new_book: AddBook,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("handling add book");

    let create_book_dto = new_book.to_dto();

    let id_response = db_manager.create_book(create_book_dto).map(|book|
        { IdResponse::new(book.id) }
    );

    respond(id_response, warp::http::StatusCode::CREATED)
}

pub async fn update_status(
    book_id: i64,
    db_manager: DBAccessManager,
    status_update: UpdateStatus,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("handling update status");

    let id_response = db_manager.update_book_status(book_id, status_update.status).map(|_|
        { IdResponse::new(book_id) }
    );

    respond(id_response, warp::http::StatusCode::OK)
}

pub async fn delete_book(
    book_id: i64,
    db_manager: DBAccessManager,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("handling delete book");

    let result = db_manager.delete_book(book_id).map(|_| -> () {()});

    respond(result, warp::http::StatusCode::NO_CONTENT)
}

pub async fn list_books(
    db_manager: DBAccessManager,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("handling list books");

    let result = db_manager.list_books();

    respond(result, warp::http::StatusCode::OK)
}

fn respond<T: Serialize>(result: Result<T, AppError>, status: warp::http::StatusCode) -> Result<impl warp::Reply, warp::Rejection> {
    match result {
        Ok(response) => {
            Ok(warp::reply::with_status(warp::reply::json(&response), status))
        }
        Err(err) => {
            log::error!("Error while trying to respond: {}", err.to_string());
            Err(warp::reject::custom(err))
        }
    }
}
