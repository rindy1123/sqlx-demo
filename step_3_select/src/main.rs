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

    let books = sqlx::query_as!(
        Book,
        r#"
            SELECT * FROM books WHERE author_id = $1
        "#,
        author.id
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    println!("Books: {:#?}", books)
}
