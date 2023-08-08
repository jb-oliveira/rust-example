use std::sync::Mutex;

use actix_web::{get,App, HttpServer, web};
use serde::{Deserialize, Serialize};

struct AppState {
    todo_list: Mutex<Vec<Todo>>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    id: i32,
    date: i64,
    title: String,
}

#[get("/")]
async fn index() -> String {
    "Hello world".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState {
        todo_list: Mutex::new(vec![])
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(index)
    })
        .bind(("localhost", 8080))?
        .run()
        .await
}

