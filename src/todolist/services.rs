use super::models::{CreateEntryData, UpdateEntryData};
use crate::{AppState, TodoListEntry};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

#[get("todolist/entries")]
async fn get_entries(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(data.todolist_entries.lock().unwrap().to_vec())
}

#[post("/todolist/entries")]
async fn create_entry(
    data: web::Data<AppState>,
    pram_obj: web::Json<CreateEntryData>,
) -> impl Responder {
    let mut todolist_entries = data.todolist_entries.lock().unwrap();
    let mut max_id: i32 = 0;
    for i in 0..todolist_entries.len() {
        if todolist_entries[i].id > max_id {
            max_id = todolist_entries[i].id
        }
    }

    todolist_entries.push(TodoListEntry {
        id: max_id + 1,
        title: pram_obj.title.clone(),
        date: pram_obj.date,
    });

    println!("{:?}", pram_obj);
    HttpResponse::Ok().json(todolist_entries.to_vec())
}

#[put("todolist/entries/{id}")]
async fn update_entries(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    param_obj: web::Json<UpdateEntryData>,
) -> impl Responder {
    // pull value from the path
    let id = path.into_inner();
    let mut todolist_entries = data.todolist_entries.lock().unwrap();
    for i in 0..todolist_entries.len() {
        if todolist_entries[i].id == id {
            todolist_entries[i].title = param_obj.title.clone();
            break;
        }
    }
    HttpResponse::Ok().json(todolist_entries.to_vec())
}

#[delete("todolist/entries/{id}")]
async fn delete_entries(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let mut todolist_entries = data.todolist_entries.lock().unwrap();
    let id = path.into_inner();
    *todolist_entries = todolist_entries
        .to_vec()
        .into_iter()
        .filter(|x| x.id != id)
        .collect();

    HttpResponse::Ok().json(todolist_entries.to_vec())
}

// Path Service func to main
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_entries)
        .service(create_entry)
        .service(update_entries)
        .service(delete_entries);
}
