use std::env;

use dotenvy::dotenv;
use sqlx::{
    postgres::{PgHasArrayType, PgPoolOptions, PgTypeInfo},
    types::chrono::{DateTime, Utc},
    FromRow,
};

#[allow(dead_code)]
#[derive(Debug, FromRow)]
struct Author {
    id: i32,
    name: String,
    age: i32,
    books: Vec<Book>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[allow(dead_code)]
#[derive(Debug, FromRow, sqlx::Type)]
struct Book {
    id: i32,
    title: String,
    author_id: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl PgHasArrayType for Book {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_books")
    }
}

#[tokio::main]
async fn main() {
    dotenv().unwrap();
    let database_url = env::var("DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    // Create a new author
    let author = sqlx::query!(
        r#"
            INSERT INTO authors ( name, age )
            VALUES ( $1, $2 )
            RETURNING *
        "#,
        "J.K. Rowling",
        58
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    // Create a new book
    sqlx::query!(
        r#"
            INSERT INTO books ( title, author_id )
            VALUES ( $1, $2 )
        "#,
        "Harry Potter",
        author.id
    )
    .execute(&pool)
    .await
    .unwrap();

    // Create another book
    sqlx::query!(
        r#"
            INSERT INTO books ( title, author_id )
            VALUES ( $1, $2 )
        "#,
        "Fantastic Beasts",
        author.id
    )
    .execute(&pool)
    .await
    .unwrap();

    // Fetch an author
    let author: Author = sqlx::query_as(
        r#"
            SELECT a.*, ARRAY_REMOVE(ARRAY_AGG(b.*), NULL) books FROM authors a
            LEFT OUTER JOIN books b ON a.id = b.author_id
            WHERE a.id = $1
            GROUP BY a.id
        "#,
    )
    .bind(author.id)
    .fetch_one(&pool)
    .await
    .unwrap();
    println!("{:#?}", author)
}
