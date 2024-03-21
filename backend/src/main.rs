use sqlx::postgres::PgPoolOptions;
use std::{env, path::PathBuf};
use rocket::{fairing::{Fairing, Info, Kind}, http::Header, response::status, serde::json::Json, Request, Response};

#[macro_use]
extern crate rocket;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, OPTIONS, PATCH, PUT, DELETE"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Todo {
    id: Option<i32>,
    name: String,
    is_complete: bool
}


#[get("/todos")]
async fn todos_get(db: &rocket::State<sqlx::PgPool>) -> String {
    let users = sqlx::query_as!(Todo, "SELECT * FROM todos ORDER BY id ASC").fetch_all(db.inner()).await.expect("querry failed");
    let json = serde_json::to_string(&users).expect("failed to convert to json");
    json
}

#[post("/todos", data ="<todo>")]
async fn todos_post(db: &rocket::State<sqlx::PgPool>, todo: Json<Todo>) -> Json<Todo> {
    let resp = sqlx::query_as!(Todo, "INSERT INTO todos (name, is_complete) VALUES ($1, $2) RETURNING id, name, is_complete", todo.name, false).fetch_one(db.inner()).await.expect("failed to add todo");
    Json(resp)
}


#[options("/<path..>")]
async fn options_preflight(db: &rocket::State<sqlx::PgPool>, path: PathBuf)  {
}

#[delete("/todos/<id>")]
async fn todos_delete(db: &rocket::State<sqlx::PgPool>, id: i32) {
    println!("{}",id);
    let _ = sqlx::query!("DELETE FROM todos WHERE id = $1", id).execute(db.inner()).await.expect("failed to delete todo");
}

#[patch("/todos", data = "<todo>")]
async fn todos_patch(db: &rocket::State<sqlx::PgPool>, todo: Json<Todo>) {
    let _ = sqlx::query!("UPDATE todos SET is_complete = $1 WHERE id = $2", todo.is_complete, todo.id.unwrap()).execute(db.inner()).await.expect("failed to patch todo");
}

#[launch]
async fn rocket() -> _ {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("no database url");

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    rocket::build().attach(CORS).mount("/", routes![todos_post, todos_get, todos_delete, options_preflight, todos_patch]).manage(pool)
}
