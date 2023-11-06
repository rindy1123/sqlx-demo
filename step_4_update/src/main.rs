use std::env;

use dotenvy::dotenv;
use sqlx::{
    postgres::PgPoolOptions,
    types::chrono::{DateTime, Utc},
};

#[allow(dead_code)]
#[derive(Debug)]
struct Book {
    id: i32,
    title: String,
    author_id: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
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
    let book = sqlx::query!(
        r#"
            INSERT INTO books ( title, author_id )
            VALUES ( $1, $2 )
            RETURNING *
        "#,
        "Harry Potter",
        author.id
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    // Update the book
    sqlx::query!(
        r#"
            UPDATE books
            SET title = $1
            WHERE id = $2
        "#,
        "Harry Potter and the Philosopher's Stone",
        book.id
    )
    .execute(&pool)
    .await
    .unwrap();

    let harry_potter = sqlx::query_as!(
        Book,
        r#"
            SELECT * FROM books WHERE id = $1
        "#,
        book.id
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    println!("{:#?}", harry_potter)
}
