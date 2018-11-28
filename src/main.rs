#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;

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
    fullname: String,
    email: String
}

#[get("/")]
fn index() -> &'static str {
    "It works!
    "
}

// http --json --verbose POST :8000/jsontext "fullname=Richard Gomes" email=rgomes.info@gmail.com
#[post("/jsontext", format = "json", data = "<user>")]
fn jsontext(user: Json<User>) -> String {
    format!("{}<{}>", user.fullname, user.email)
}

// http --json --verbose POST :8000/json "fullname=Richard Gomes" email=rgomes.info@gmail.com
#[post("/jsonjson", format = "json", data = "<user>")]
fn jsonjson(user: Json<User>) -> JsonValue {
    //json!("{}<{}>", user.fullname, user.email)
        json!({
            "fullname": user.fullname,
            "email": user.email
        })
}

// http --form --verbose POST :8000/form "fullname=Richard Gomes" email=rgomes.info@gmail.com
#[post("/form", data = "<user>")]
fn form(user: Form<User>) -> String {
    format!("{}<{}>", user.fullname, user.email)
}

#[post("/upload", format = "plain", data = "<data>")]
fn upload(data: Data) -> io::Result<String> {
    data.stream_to_file(env::temp_dir().join("upload.txt")).map(|n| n.to_string())
}

fn main() {
    rocket::ignite()
        //FIXME: .attach(Template::fairing())
        .mount("/", routes![index, jsontext, jsonjson, form, upload])
        .launch();
}
