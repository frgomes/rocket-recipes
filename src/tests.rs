use rocket::local::Client;
use rocket::http::{Status, ContentType};

use std::env;
use std::io::Read;
use std::fs::{self, File};

#[test]
fn test_index() {
    let rocket = rocket::ignite().mount("/", routes![super::index]);
    let client = Client::new(rocket).unwrap();
    let mut response = client.get("/").dispatch();
    assert_eq!(response.body_string(), Some(super::index().to_string()));
}
