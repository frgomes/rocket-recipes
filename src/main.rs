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
    fullname: String,
    email   : String,
    age     : i32,
}

#[get("/")]
fn index() -> &'static str {
    "It works!
    "
}

// http --json --verbose POST :8000/jsontext "fullname=John Doe" email=john.doe@example.com age=35
#[post("/jsontext", format = "json", data = "<user>")]
fn jsontext(user: Json<User>) -> String {
    format!("{}<{}> is {} years old", user.fullname, user.email, user.age)
}

// http --json --verbose POST :8000/jsonjson "fullname=John Doe" email=john.doe@example.com age=35
#[post("/jsonjson", format = "json", data = "<user>")]
fn jsonjson(user: Json<User>) -> Json<User> {
    user
}

// http --form --verbose POST :8000/formtext "fullname=John Doe" email=john.doe@example.com age=35
#[post("/formtext", data = "<user>")]
fn formtext(user: Form<User>) -> String {
    format!("{}<{}> is {} years old", user.fullname, user.email, user.age)
}

#[post("/upload", format = "plain", data = "<data>")]
fn upload(data: Data) -> io::Result<String> {
    data.stream_to_file(env::temp_dir().join("upload.txt")).map(|n| n.to_string())
}

fn main() {
    rocket::ignite()
        //FIXME: .attach(Template::fairing())
        .mount("/", routes![index, jsontext, jsonjson, formtext, upload])
        .launch();
}
