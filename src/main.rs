use actix_web::{get, web, App, HttpServer};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

mod todolist;
use todolist::services::config;

// share state to pass to route handler
// gonna use as DB now
struct AppState {
    todolist_entries: Mutex<Vec<TodoListEntry>>,
}

// Allow to use in json and duplicate
#[derive(Serialize, Deserialize, Clone)]
struct TodoListEntry {
    id: i32,
    date: i64,
    title: String,
}

#[get("/")]
async fn index() -> String {
    "This is a health check route".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState {
        todolist_entries: Mutex::new(vec![]),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(index)
            .configure(config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
