#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket::Data;
use rocket::request::{Form, LenientForm};
use rocket::response::{content, Stream};
use rocket_contrib::json::{Json, JsonValue};
use std::env;
use std::io::{self, repeat, Repeat, Read, Take};

#[cfg(test)] mod tests;

#[derive(FromForm)]
#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    realname: String,
    email   : String,
    age     : i32,
}

#[get("/")]
fn index() -> &'static str {
    "It works!
    "
}

// http --verbose POST :8000/texttext username=jsmith
#[post("/texttext", data = "<username>")]
fn texttext(username: String) -> String {
    username
}

// http --verbose POST :8000/textjson username=jsmith
#[post("/textjson", data = "<username>")]
fn textjson(username: String) -> Json<User> {
    Json(User { username: username, realname: "(unknown)".into(), email: "(unknown)".into(), age: 0 })
}

// http --json --verbose POST :8000/jsontext "realname=John Smith" email=john.smith@example.com age=35
#[post("/jsontext", format = "json", data = "<user>")]
fn jsontext(user: Json<User>) -> String {
    format!(
        "username: {}
         realname:{}
         email: {}
         age: {}", user.username, user.realname, user.email, user.age)
}

// http --json --verbose POST :8000/jsonjson "realname=John Smith" email=john.smith@example.com age=35
#[post("/jsonjson", format = "json", data = "<user>")]
fn jsonjson(user: Json<User>) -> Json<User> {
    user
}

// http --form --verbose POST :8000/formtext "realname=John Smith" email=john.smith@example.com age=35
#[post("/formtext", data = "<user>")]
fn formtext(user: Form<User>) -> String {
    format!(
        "username: {}
         realname:{}
         email: {}
         age: {}", user.username, user.realname, user.email, user.age)
}

// http --form --verbose POST :8000/formjson "realname=John Smith" email=john.smith@example.com age=35
#[post("/formjson", data = "<user>")]
fn formjson(user: Form<User>) -> Json<User> {
    Json(user.into_inner())
}

#[post("/upload", format = "plain", data = "<data>")]
fn upload(data: Data) -> io::Result<String> {
    data.stream_to_file(env::temp_dir().join("upload.txt")).map(|n| n.to_string())
}

fn main() {
    rocket::ignite()
        //FIXME: .attach(Template::fairing())
        .mount("/", routes![index, texttext, textjson, jsontext, jsonjson, formtext, upload])
        .launch();
}
