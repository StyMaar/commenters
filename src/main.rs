#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate rocket_contrib;

#[macro_use] extern crate serde_derive;

extern crate rocket_cors;

use rocket::response::status::Accepted;

use rocket::http::Method;
use rocket_cors::{AllowedOrigins};

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;


extern crate dotenv;

extern crate chrono;
extern crate uuid;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use models::{NewComment, Comment};

use uuid::Uuid;
use chrono::prelude::Utc;

#[derive(Deserialize)]
struct PartialComment {
    text: String,
    author: String,
}

extern crate rocket;

use rocket_contrib::{Json};

#[post("/<article>/comments", format = "application/json", data = "<comment>")]
fn new_comment(article: String, comment: Json<PartialComment>) -> Accepted<Json<Comment>> {

    let uuid = Uuid::new_v4();
    let date = format!("{:?}",Utc::now()); // Javascript n'arrive pas à parser le résultat de Utc::to_string(), par contre il arrive bien à parser la valeur renvoyé par l'implem de Debug par Utc. Allez comprendre …
    let db_comment = NewComment{
        article: &article,
        message: &comment.text,
        author: &comment.author,
        date: &date,
        uuid: &uuid.simple().to_string(),
    };
    save_comment(&db_comment);
    Accepted(Some(Json(db_comment.to_comment())))
}

#[get("/<article>/comments", format = "application/json", rank = 2)]
fn list_comments(article: String) -> Json<Vec<Comment>> {
    fetch_comments(&article)
}

use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

#[get("/static/<path..>")]
fn static_assets(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}

fn main() {

    let (allowed_origins, failed_origins) = AllowedOrigins::some(&["http://localhost:3000"]);
    assert!(failed_origins.is_empty());

    let options = rocket_cors::Cors {
        allowed_origins: allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post].into_iter().map(From::from).collect(),
        ..Default::default()
    };

    rocket::ignite().mount("/", routes![new_comment, list_comments, static_assets]).attach(options).launch();
}

// curl 'http://localhost:8000/poney/comments' -H 'Accept: */*' --compressed -H 'Content-Type: application/json' --data '{"author":"yolo@hello.example.org", "text":"yolo !!!"}'

fn save_comment(comment : &NewComment) {
    let connection = establish_connection();

    diesel::insert(comment).into(schema::comments::table)
    .execute(&connection)
    .expect("Error saving new comments");
}

fn fetch_comments(art: &str) -> Json<Vec<Comment>>{
    let connection = establish_connection();

    use schema::comments::dsl;

    let results = dsl::comments
        .filter(dsl::article.eq(art))
        .load::<Comment>(&connection)
        .expect("Error loading posts");
    Json(results)

}

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
    .expect(&format!("Error connecting to {}", database_url))
}
