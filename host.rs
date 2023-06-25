// main.rs
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response::content::Html;
use rocket::response::NamedFile;
use std::path::PathBuf;

//use image;
extern crate image;
use image::{GenericImageView, ImageBuffer, Rgba};
//use image::GenericImageView;

fn modify_img() {
 let img = image::open("/app/static/image.jpg").expect("File not found!");
 let (w, h) = img.dimensions();
 let mut output = ImageBuffer::new(w, h); // create a new buffer for our output
 for (x, y, pixel) in img.pixels() {
   output.put_pixel(x, y, Rgba([
   255 - pixel[0],
   255 - pixel[1],
   255 - pixel[2],
   pixel[3],
  ]));
 }
 output.save("/app/static/image.jpg").expect("Failed to save image");
}

//fn getImg() {
//}

#[get("/")]
fn index() -> Html<&'static str> {
    Html("<html><body><h1>Hello, World!</h1><img src=\"/image.jpg\"></body></html>")
}

#[get("/image.jpg")]
fn show_image() -> Option<NamedFile> {
    let path: PathBuf = ["static", "image.jpg"].iter().collect();
    NamedFile::open(path).ok()
}	

fn main() {
    modify_img();
    rocket::ignite()
        .mount("/", routes![index, show_image])
        .launch();
}
