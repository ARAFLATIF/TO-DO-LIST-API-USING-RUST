use warp::Filter;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
struct Todo {
    id: u64,
    text: String,
}

type Db = Arc<Mutex<Vec<Todo>>>;

#[tokio::main]
async fn main() {
    let db: Db = Arc::new(Mutex::new(Vec::new()));

    let db_filter = warp::any().map(move || db.clone());

    let add_todo = warp::path("todos")
        .and(warp::post())
        .and(warp::body::json())
        .and(db_filter.clone())
        .and_then(add_todo);

    let list_todos = warp::path("todos")
        .and(warp::get())
        .and(db_filter.clone())
        .and_then(list_todos);

    let delete_todo = warp::path!("todos" / u64)
        .and(warp::delete())
        .and(db_filter.clone())
        .and_then(delete_todo);

    let routes = add_todo.or(list_todos).or(delete_todo);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn add_todo(new_todo: Todo, db: Db) -> Result<impl warp::Reply, warp::Rejection> {
    let mut todos = db.lock().unwrap();
    todos.push(new_todo);
    Ok(warp::reply::with_status("Todo added", warp::http::StatusCode::CREATED))
}

async fn list_todos(db: Db) -> Result<impl warp::Reply, warp::Rejection> {
    let todos = db.lock().unwrap();
    Ok(warp::reply::json(&*todos))
}

async fn delete_todo(id: u64, db: Db) -> Result<impl warp::Reply, warp::Rejection> {
    let mut todos = db.lock().unwrap();
    if let Some(pos) = todos.iter().position(|todo| todo.id == id) {
        todos.remove(pos);
        Ok(warp::reply::with_status("Todo deleted", warp::http::StatusCode::OK))
    } else {
        Ok(warp::reply::with_status("Todo not found", warp::http::StatusCode::NOT_FOUND))
    }
}
