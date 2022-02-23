#[macro_use] extern crate rocket;

use rocket_contrib::serve::StaticFiles;


#[get("/")]
fn world() -> &'static str {
    "Hello, world!"
}

#[get("/")]
fn hello() -> String {
    format!("Hello page!")
}

#[get("/<name>")]
fn hello_name(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[catch(404)]
fn not_found() -> String {
    format!("oy m8")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![world])
        .mount("/hello", routes![hello, hello_name])
        .register("/", catchers![not_found])

}