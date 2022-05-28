use crate::model::{Todo, TodoList};
use rocket_db_pools::{sqlx, Database};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

#[derive(Database)]
#[database("projectdb")]
pub struct ProjectDbConn(sqlx::PgPool);

pub async fn establish_connection(db_url: &str) -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(1)
        .connect(db_url)
        .await
        .expect(&format!("Error connecting to {}", db_url))
}

pub async fn run_migrations(conn: &Pool<Postgres>) {
    sqlx::migrate!().run(conn).await.unwrap();
}

pub async fn add_todo(db: &ProjectDbConn, todo: Todo) {
    sqlx::query(
        "\
    INSERT INTO todo (task) \
    VALUES ($1) \
    ",
    )
    .bind(todo.task)
    .execute(&db.0)
    .await
    .unwrap();
}

pub async fn get_todos(db: &ProjectDbConn) -> TodoList {
    let todos = sqlx::query_as::<_, Todo>(
        "\
        SELECT task \
        FROM todo \
        ",
    )
    .fetch_all(&db.0)
    .await
    .unwrap();

    TodoList { todos }
}
