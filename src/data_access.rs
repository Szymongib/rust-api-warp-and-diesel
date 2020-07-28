use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use crate::model::{CreateBookDTO, BookDTO, BookStatus};
use crate::errors::{AppError,ErrorType};

type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;

pub struct DBAccessManager {
    connection: PooledPg,
}

impl DBAccessManager {
    pub fn new(connection: PooledPg) -> DBAccessManager {
        DBAccessManager {connection}
    }

    pub fn create_book(&self, dto: CreateBookDTO) -> Result<BookDTO, AppError> {
        use super::schema::books;

        diesel::insert_into(books::table) // insert into books table
            .values(&dto) // use values from CreateBookDTO
            .get_result(&self.connection) // execute query
            .map_err(|err| {
                AppError::from_diesel_err(err, "while creating book")
            }) // if error occurred map it to AppError
    }

    pub fn list_books(&self) -> Result<Vec<BookDTO>, AppError> {
        use super::schema::books::dsl::*;

        books
            .load(&self.connection)
            .map_err(|err| {
                AppError::from_diesel_err(err, "while listing books")
            })
    }

    pub fn update_book_status(&self, book_id: i64, new_status: BookStatus) -> Result<usize, AppError> {
        use super::schema::books::dsl::*;

        let updated = diesel::update(books)
            .filter(id.eq(book_id))
            .set(status.eq(new_status))
            .execute(&self.connection)
            .map_err(|err| {
                AppError::from_diesel_err(err, "while updating book status")
            })?;

        if updated == 0 {
            return Err(AppError::new("Book not found", ErrorType::NotFound))
        }
        return Ok(updated)
    }

    pub fn delete_book(&self, book_id: i64) -> Result<usize, AppError> {
        use super::schema::books::dsl::*;

        let deleted = diesel::delete(books.filter(id.eq(book_id)))
            .execute(&self.connection)
            .map_err(|err| {
                AppError::from_diesel_err(err, "while deleting book")
            })?;

        if deleted == 0 {
            return Err(AppError::new("Book not found", ErrorType::NotFound))
        }
        return Ok(deleted)
    }
}

