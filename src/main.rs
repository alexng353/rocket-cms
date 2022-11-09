#[macro_use]
extern crate rocket;

use std::{fs::read_to_string, path::Path, path::PathBuf};

use rocket::{
    fs::NamedFile,
    response::{self, content::RawHtml},
    Request,
};

#[get("/")]
fn index() -> response::content::RawHtml<String> {
    let html = read_to_string("templates/index.html").unwrap();
    RawHtml(html)
}

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    // NamedFile::open(Path::new("static/").join(file)).await.ok()
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("I couldn't find '{}'. Try something else?", req.uri())
}

#[launch]
fn rocket() -> _ {
    println!("Server started on port 27000");

    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![files])
        .register("/", catchers![not_found])
}
