use std::env;

use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;

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
    println!("author: {}, book: {}", author.name, book.title)
}
