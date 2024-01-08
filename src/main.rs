pub mod todos_hx;
use crate::todos_hx::{initial_todos, todo_service, Todo};
use actix_cors::Cors;
use actix_files::NamedFile;
use actix_web::{get, web::Data, App, HttpServer, Responder};
use std::sync::Mutex;

struct State {
    todos: Mutex<Vec<Todo>>,
}

#[get("/")]
async fn index() -> impl Responder {
    NamedFile::open_async("./src/public/index.html").await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = Data::new(State {
        todos: Mutex::new(initial_todos()),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(state.clone())
            .service(index)
            .service(todo_service())
    })
    .bind(("127.0.0.1", 3030))?
    .run()
    .await
}
