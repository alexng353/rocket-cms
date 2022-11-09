#[macro_use]
extern crate rocket;
extern crate hex;

use rocket::{
    fs::FileServer,
    http::{Cookie, CookieJar},
    response::{self, content::RawHtml},
    serde::json::Json,
    serde::Deserialize,
};
use sha2::{Digest, Sha256};
use std::fs::read_to_string;

#[catch(404)]
fn not_found() -> response::content::RawHtml<String> {
    let html: String = read_to_string("templates/404.html").unwrap();
    RawHtml(html)
}

// fn not_found(req: &Request) -> String {
//     format!("I couldn't find '{}'. Try something else?", req.uri())
// }
// index page
#[get("/")]
fn index() -> response::content::RawHtml<String> {
    let html: String = read_to_string("templates/index.html").unwrap();
    RawHtml(html)
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct User<'r> {
    email: &'r str,
    password: &'r str,
}

#[post("/admin", data = "<user>")]
fn admin(user: Json<User<'_>>) -> String {
    println!("email: {}", user.email);
    println!("password: {}", user.password);

    // hash password to sha256

    let mut hasher = Sha256::new();
    hasher.update(user.password);
    let result = hasher.finalize();
    let hex: String = hex::encode(result);

    println!("hex: {}", hex);

    // read data.json

    let data: String = read_to_string("data.json").unwrap();

    let json: serde_json::Value = serde_json::from_str(&data).unwrap();

    let mut success: bool = false;

    for i in json.as_array().unwrap() {
        let email = i["email"].as_str().unwrap();
        let password = i["password"].as_str().unwrap();
        let name = i["name"].as_str().unwrap();

        if email == user.email && password == hex {
            println!("Welcome, {}!", name);

            success = true;
        }
    }

    // println!(
    //     "{:#?}",
    //     cookies
    //         .get("message")
    //         .map(|crumb| format!("Message: {}", crumb.value()))
    // );

    if success == false {
        println!("Invalid credentials");
        return r#"{
            "status": "not ok"
        }"#
        .to_string();
    } else {
        return r#"{
            "status": "ok"
        }"#
        .to_string();
    }
}

#[get("/admin")]
fn admin_page() -> response::content::RawHtml<String> {
    let html = read_to_string("templates/admin.html").unwrap();
    RawHtml(html)
}

#[get("/test")]
fn user_id(cookies: &CookieJar<'_>) -> String {
    // set a cookie called "name" with a value of "value"
    cookies.add(Cookie::new("name", "value"));

    // get the value of the "name" cookie

    "Hello, world!".to_string()
}

#[get("/xd")]
fn get_user_id(cookies: &CookieJar<'_>) -> Option<String> {
    // set a cookie called "name" with a value of "value"

    // get the value of the "name" cookie

    cookies.get("name").map(|crumb| crumb.value().to_string())
}

#[launch]
fn rocket() -> _ {
    println!("Server started on port 27000");

    rocket::build()
        .mount("/", FileServer::from("static/"))
        .mount("/", routes![index])
        .mount("/", routes![user_id])
        .mount("/", routes![get_user_id])
        // .mount("/", routes![files])
        // .mount("/", routes![admin])
        .mount("/", routes![admin])
        .mount("/", routes![admin_page])
        .register("/", catchers![not_found])
}
