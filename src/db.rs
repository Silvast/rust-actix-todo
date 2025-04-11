use log::{info, error};
use sqlx::postgres::{PgPoolOptions, PgPool};
use sqlx::migrate::MigrateDatabase;
use sqlx::{Pool, Postgres, Connection};
use std::env;
use serde::{Serialize, Deserialize};

pub async fn setup_database() -> Result<PgPool, Box<dyn std::error::Error>> {
    dotenv::from_path(".env").ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5433/todos".to_string());

    if !Postgres::database_exists(&database_url).await.unwrap_or(false) {
        info!("Database does not exist, creating...");
        Postgres::create_database(&database_url).await?;
        info!("Database created successfully.");
    } else {
        info!("Database already exists.");
    }

    info!("Connecting to database and creating pool with URL: {}", database_url);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    info!("Running migrations using sqlx::migrate!...");
    sqlx::migrate!("./migrations").run(&pool).await?;
    info!("SQLx migrations applied successfully.");

    Ok(pool)
}

pub async fn run_test_query(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    info!("Running test query...");
    let row: (i32,) = sqlx::query_as("SELECT 1")
        .fetch_one(pool)
        .await?;
        
    println!("Query result: {}", row.0);
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ToDo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

pub async fn get_todo_by_id(pool: &PgPool, id: i32) -> Result<Option<ToDo>, Box<dyn std::error::Error>> {
    let todo = sqlx::query_as::<_, ToDo>("SELECT id, title, completed FROM todo WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?;
    
    Ok(todo)
}

// Add a struct for inserting new todo items
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateToDo {
    pub title: String,
    pub completed: bool,
}

pub async fn add_todo(pool: &PgPool, new_todo: CreateToDo) -> Result<ToDo, Box<dyn std::error::Error>> {
    let todo = sqlx::query_as::<_, ToDo>(
        "INSERT INTO todo (title, completed) VALUES ($1, $2) RETURNING id, title, completed"
    )
    .bind(new_todo.title)
    .bind(new_todo.completed)
    .fetch_one(pool)
    .await?;
    
    Ok(todo)
}

pub async fn mark_todo_completed(pool: &PgPool, id: i32) -> Result<Option<ToDo>, Box<dyn std::error::Error>> {
    let todo = sqlx::query_as::<_, ToDo>(
        "UPDATE todo SET completed = true WHERE id = $1 RETURNING id, title, completed"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    
    Ok(todo)
} 

pub async fn get_all_todos(pool: &PgPool) -> Result<Vec<ToDo>, Box<dyn std::error::Error>> {
    let todos = sqlx::query_as::<_, ToDo>("SELECT id, title, completed FROM todo ORDER BY id")
        .fetch_all(pool)
        .await?;
    
    Ok(todos)
}