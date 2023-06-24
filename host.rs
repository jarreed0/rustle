// main.rs
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response::content::Html;
use rocket::response::NamedFile;
use std::path::PathBuf;

#[get("/")]
fn index() -> Html<&'static str> {
    Html("<html><body><h1>Hello, Rocket!</h1><img src=\"/image.jpg\"></body></html>")
}

#[get("/image.jpg")]
fn image() -> Option<NamedFile> {
    let path: PathBuf = ["static", "image.jpg"].iter().collect();
    NamedFile::open(path).ok()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, image])
        .launch();
}
