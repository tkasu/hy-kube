use crate::model::{Todo, TodoList};
use rocket::{fairing, Build, Rocket};
use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("projectdb")]
pub struct ProjectDbConn(sqlx::PgPool);

pub async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    if let Some(db) = ProjectDbConn::fetch(&rocket) {
        sqlx::migrate!().run(&db.0).await.unwrap();
        Ok(rocket)
    } else {
        Err(rocket)
    }
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
        ORDER BY id DESC \
        LIMIT 50 \
        ",
    )
    .fetch_all(&db.0)
    .await
    .unwrap();

    TodoList { todos }
}
