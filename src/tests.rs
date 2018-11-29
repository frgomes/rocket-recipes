use rocket::Rocket;
use rocket::local::{Client, LocalRequest, LocalResponse};
use rocket::http::{Status, ContentType};
use rocket_contrib::json::Json;
use super::User;

use std::env;
use std::io::Read;
use std::fs::{self, File};

#[test]
fn test_index() {
    let rocket: Rocket = rocket::ignite().mount("/", routes![super::index]);
    let client: Client = Client::new(rocket).unwrap();
    let mut response: LocalResponse = client.get("/").dispatch();
    assert_eq!(response.body_string(), Some(super::index().to_string()));
}

#[test]
fn test_jsontext() {
    let user: User = User { fullname: "John Doe".into(), email: "john.doe@example.com".into(), age: 35 };
    let serialized: String = serde_json::to_string(&user).unwrap();

    let rocket: Rocket = rocket::ignite().mount("/", routes![super::jsontext]);
    let client: Client = Client::new(rocket).unwrap();
    let request: LocalRequest =
        client
            .post("/jsontext")
            .header(ContentType::JSON)
            .body(serialized);
    let mut response: LocalResponse = request.dispatch();
    assert_eq!(response.body_string(), Some(super::jsontext(Json(user)).to_string()) );
}
