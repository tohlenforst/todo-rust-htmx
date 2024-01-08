use crate::State;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Scope};
use html_to_string_macro::html;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FormData {
    #[serde(rename = "newTodo")]
    pub new_todo: String,
}

#[derive(Clone)]
pub struct Todo {
    pub id: i32,
    pub text: String,
    pub completed: bool,
}

fn todo_html(todo: &Todo) -> String {
    let id = todo.id.to_string();
    let text = todo.text.clone();
    let completed = todo.completed.to_string();
    let hx_put = String::from("/api/todos/") + &todo.id.to_string();
    let hx_delete = String::from("/api/todos/") + &todo.id.to_string();
    html!(
        <div style="border: 1px solid black;" hx-target="closest div" hx-swap="outerHTML">
            <div>"ID: "{id}</div>
            <div>"Text: "{text}</div>
            <div class="status">"Completed?: "{completed}</div>
            <button hx-put={hx_put}>"Toggle Completed"</button>
            <button hx-delete={hx_delete}>"Delete"</button>
        </div>
    )
}

#[get("/")]
async fn get_todos(data: web::Data<State>) -> impl Responder {
    let todos = data.todos.lock().unwrap();
    let doc = html!(
        <div>{todos.iter().map(todo_html).collect::<Vec<String>>().join("")}</div>
    );
    HttpResponse::Ok().body(doc)
}

#[post("/")]
async fn add_todo(form: web::Form<FormData>, data: web::Data<State>) -> impl Responder {
    let mut todos = data.todos.lock().unwrap();
    let new_todo = Todo {
        id: match todos.last().clone() {
            Some(todo) => todo.id + 1,
            None => 1,
        },
        text: form.new_todo.clone(),
        completed: false,
    };
    todos.push(new_todo.clone());
    HttpResponse::Ok().body(todo_html(&new_todo))
}

#[put("/{id}")]
async fn update_todo(path: web::Path<i32>, data: web::Data<State>) -> impl Responder {
    let id = path.into_inner();
    let mut todos = data.todos.lock().unwrap();
    let target = todos.iter_mut().find(|todo| todo.id == id);
    HttpResponse::Ok().body(match target {
        Some(updated_todo) => {
            updated_todo.completed = !updated_todo.completed;
            todo_html(&updated_todo)
        }
        None => String::from(""),
    })
}

#[delete("/{id}")]
async fn delete_todo(path: web::Path<i32>, data: web::Data<State>) -> impl Responder {
    let id = path.into_inner();
    let mut todos = data.todos.lock().unwrap();
    todos.retain(|todo| todo.id != id);
    HttpResponse::Ok().body("")
}

pub fn initial_todos() -> Vec<Todo> {
    vec![
        Todo {
            id: 1,
            text: String::from("Learn React"),
            completed: true,
        },
        Todo {
            id: 2,
            text: String::from("Learn Redux"),
            completed: false,
        },
        Todo {
            id: 3,
            text: String::from("Build something fun!"),
            completed: false,
        },
    ]
}

pub fn todo_service() -> Scope {
    web::scope("/api/todos")
        .service(get_todos)
        .service(add_todo)
        .service(update_todo)
        .service(delete_todo)
}
