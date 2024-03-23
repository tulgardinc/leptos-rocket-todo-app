use std::{env, path::PathBuf};
use rocket::{fairing::{Fairing, Info, Kind}, fs::FileServer, http::Header, serde::json::Json, Request, Response};
use sqlx::Sqlite;

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
    id: Option<i64>,
    name: String,
    is_complete: bool
}

//#[get("/")]
//async fn index(db: &rocket::State<sqlx::Pool<Sqlite>>) -> rocket::fs::NamedFile {
//    let dist_dir = env::var("DIST_DIR").expect("failed to get dist url");
//    rocket::fs::NamedFile::open("./dist/index.html").await.expect("failed to open index.html")
//}

#[get("/todos")]
async fn todos_get(db: &rocket::State<sqlx::Pool<Sqlite>>) -> String {
    let users = sqlx::query_as!(Todo, "SELECT * FROM todos ORDER BY id ASC").fetch_all(db.inner()).await.expect("querry failed");
    let json = serde_json::to_string(&users).expect("failed to convert to json");
    json
}

#[post("/todos", data ="<todo>")]
async fn todos_post(db: &rocket::State<sqlx::Pool<Sqlite>>, todo: Json<Todo>) -> Json<Todo> {
    let resp = sqlx::query_as!(Todo, "INSERT INTO todos (name, is_complete) VALUES ($1, $2) RETURNING id, name, is_complete", todo.name, false).fetch_one(db.inner()).await.expect("failed to add todo");
    Json(resp)
}

#[options("/<path..>")]
async fn options_preflight(db: &rocket::State<sqlx::Pool<Sqlite>>, path: PathBuf)  {
}

#[delete("/todos/<id>")]
async fn todos_delete(db: &rocket::State<sqlx::Pool<Sqlite>>, id: i32) {
    println!("{}",id);
    let _ = sqlx::query!("DELETE FROM todos WHERE id = $1", id).execute(db.inner()).await.expect("failed to delete todo");
}

#[patch("/todos", data = "<todo>")]
async fn todos_patch(db: &rocket::State<sqlx::Pool<Sqlite>>, todo: Json<Todo>) {
    let id = todo.id.unwrap();
    let _ = sqlx::query!("UPDATE todos SET is_complete = $1 WHERE id = $2", todo.is_complete, id).execute(db.inner()).await.expect("failed to patch todo");
}

#[launch]
async fn rocket() -> _ {
    let db_url = env::var("DATABASE_URL").expect("failed to get database url");
    let dist_dir = env::var("DIST_DIR").expect("failed to get dist url");

    let pool = sqlx::sqlite::SqlitePool
        ::connect(&db_url)
        .await
        .expect("Failed to create pool");

    rocket::build().attach(CORS).mount("/", FileServer::from(dist_dir))
    .mount("/", routes![todos_post, todos_get, todos_delete, options_preflight, todos_patch]).manage(pool)

}
