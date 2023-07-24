use axum::{routing::get, Router, Json};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/bar", get(foo_bar));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() {}

async fn get_foo() -> &'static str { "foo" }

async fn post_foo() {}

async fn foo_bar() -> Json<Value> {
    Json(json!({ "data": 42 }))
}