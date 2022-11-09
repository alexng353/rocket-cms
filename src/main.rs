#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "I love EduBeyond"
}

use std::{fs::read_to_string, path::Path, path::PathBuf};

use rocket::{fs::NamedFile, request::FromSegments, response::content::RawHtml, Error, Request};
#[get("/test/<test>")]
fn test(test: String) -> RawHtml<String> {
    // let test = read_to_string("./static/index.html").unwrap();
    let response;
    if test != "" {
        response = read_to_string(format!("./static/{}", test)).unwrap()
    } else {
        response = read_to_string("./static/index.html").unwrap();
    }
    RawHtml(response)
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
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![test])
        .mount("/", routes![files])
        .register("/", catchers![not_found])
}
