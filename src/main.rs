#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate rocket_contrib;

#[macro_use] extern crate serde_derive;

use rocket::response::status::Accepted;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;

extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use models::{NewComment, Comment as DbComment};


#[derive(Serialize, Deserialize)]
struct Comment {
    text: String,
    author: String,
}

extern crate rocket;

use rocket_contrib::{Json};

#[post("/<article>/comments", format = "application/json", data = "<comment>")]
fn new_comment(article: String, comment: Json<Comment>) -> Accepted<Json<Comment>> {

    let db_comment = NewComment{
        article: &article,
        message: &comment.text,
        author: &comment.author,
    };
    save_comment(db_comment);
    Accepted(None)
}

#[get("/<article>/comments", format = "application/json", rank = 2)]
fn list_comments(article: String) -> Json<Vec<DbComment>> {
    fetch_comments(&article)
}

use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

#[get("/static/<path..>")]
fn static_assets(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}

fn main() {
    rocket::ignite().mount("/", routes![new_comment, list_comments, static_assets]).launch();
}

// curl 'http://localhost:8000/poney/comments' -H 'Accept: */*' --compressed -H 'Content-Type: application/json' --data '{"author":"yolo@hello.example.org", "text":"yolo !!!"}'

fn save_comment(comment : NewComment) {
    let connection = establish_connection();

    diesel::insert(&comment).into(schema::comments::table)
    .execute(&connection)
    .expect("Error saving new comments");
}

fn fetch_comments(art: &str) -> Json<Vec<DbComment>>{
    let connection = establish_connection();

    use schema::comments::dsl;

    let results = dsl::comments
        .filter(dsl::article.eq(art))
        .load::<DbComment>(&connection)
        .expect("Error loading posts");
    Json(results)

}

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
    .expect(&format!("Error connecting to {}", database_url))
}
