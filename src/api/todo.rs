use crate::db;
use actix_web::{HttpResponse, Responder, Result, get, patch, post, web};
use sqlx::PgPool;

pub struct AppState {
    pub pool: PgPool,
}

#[get("/")]
pub async fn index(_data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/todos/{id}")]
pub async fn get_todo(path: web::Path<i32>, data: web::Data<AppState>) -> Result<HttpResponse> {
    let id = path.into_inner();

    match db::get_todo_by_id(&data.pool, id).await {
        Ok(Some(todo)) => Ok(HttpResponse::Ok().json(todo)),
        Ok(None) => Ok(HttpResponse::NotFound().body(format!("No todo found with id: {}", id))),
        Err(e) => {
            log::error!("Database error: {}", e);
            Ok(HttpResponse::InternalServerError().body("Internal server error"))
        }
    }
}

#[get("/todos")]
pub async fn get_all_todos(data: web::Data<AppState>) -> Result<HttpResponse> {
    match db::get_all_todos(&data.pool).await {
        Ok(todos) => Ok(HttpResponse::Ok().json(todos)),
        Err(e) => {
            log::error!("Failed to retrieve todos: {}", e);
            Ok(HttpResponse::InternalServerError().body("Failed to retrieve todos"))
        }
    }
}
#[post("/todos")]
pub async fn create_todo(
    data: web::Data<AppState>,
    new_todo: web::Json<db::CreateToDo>,
) -> Result<HttpResponse> {
    match db::add_todo(&data.pool, new_todo.into_inner()).await {
        Ok(todo) => Ok(HttpResponse::Created().json(todo)),
        Err(e) => {
            log::error!("Failed to create todo: {}", e);
            Ok(HttpResponse::InternalServerError().body("Failed to create todo"))
        }
    }
}

#[patch("/todos/{id}/complete")]
pub async fn complete_todo(
    path: web::Path<i32>,
    data: web::Data<AppState>,
) -> Result<HttpResponse> {
    let id = path.into_inner();

    match db::mark_todo_completed(&data.pool, id).await {
        Ok(Some(todo)) => Ok(HttpResponse::Ok().json(todo)),
        Ok(None) => Ok(HttpResponse::NotFound().body(format!("No todo found with id: {}", id))),
        Err(e) => {
            log::error!("Failed to mark todo as completed: {}", e);
            Ok(HttpResponse::InternalServerError().body("Failed to update todo"))
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
        .service(get_todo)
        .service(get_all_todos)
        .service(create_todo)
        .service(complete_todo);
}
