use serde_derive::{Deserialize, Serialize};
use crate::schema::books;

#[derive(Serialize, Debug, Clone, Queryable)]
pub struct BookDTO {
    pub id: i64,
    pub title: String,
    pub author: String,
    pub status: BookStatus,
}

// Struct for creating Book
#[derive(Debug, Clone, Insertable)]
#[table_name = "books"]
pub struct CreateBookDTO {
    pub title: String,
    pub author: String,
    pub status: BookStatus,
}

// Handling enum as a text field in the database
use diesel::serialize::{ToSql, Output, IsNull};
use diesel::pg::Pg;
use std::io::Write;
use diesel::{serialize, deserialize};
use diesel::deserialize::FromSql;
use diesel::sql_types::Text;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, AsExpression, FromSqlRow)]
#[sql_type = "Text"]
pub enum BookStatus {
    WantToRead,
    Reading,
    Finished,
    Rereading,
}

impl ToSql<Text, Pg> for BookStatus {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            BookStatus::WantToRead => out.write_all(b"WANT_TO_READ")?,
            BookStatus::Reading => out.write_all(b"READING")?,
            BookStatus::Finished => out.write_all(b"FINISHED")?,
            BookStatus::Rereading => out.write_all(b"REREADING")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<Text, Pg> for BookStatus {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"WANT_TO_READ" => Ok(BookStatus::WantToRead),
            b"READING" => Ok(BookStatus::Reading),
            b"FINISHED" => Ok(BookStatus::Finished),
            b"REREADING" => Ok(BookStatus::Rereading),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}