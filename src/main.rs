#[macro_use] extern crate rocket;

use rocket_dyn_templates::{Template};
use std::{path::PathBuf, time::SystemTime};
use std::fs;

use serde::{Serialize};

use chrono::naive::NaiveDateTime;


#[derive(Serialize)]
struct Content {
    title: String,
    date: String,
    content: String,
}



#[get("/<name..>")]
fn index(name: PathBuf) -> Template {
    
    println!(">>>> {:?}", name);
    let file_name : &str = name.to_str().unwrap_or("content/error/default");
    let file_name = format!("{}{}", "content/", file_name);
    println!(">>>> {}", file_name);

    let markdown = match fs::read_to_string(&file_name) {
        Ok(markdown) => markdown,
        Err(_e) => format!("# Woops... content unavailable 😞"),
    };

    let seconds = fs::metadata(&file_name)
        .unwrap()
        .modified().unwrap_or(SystemTime::now())
        .duration_since(SystemTime::UNIX_EPOCH).unwrap()
        .as_secs().try_into().unwrap();
    
    let date = NaiveDateTime::from_timestamp(seconds, 0)
        .format("%Y-%m-%d").to_string();

    let context = Content {
        title: format!("TODO: Some title"),
        date,
        content: markdown::to_html(&markdown),
    };

    Template::render("index", &context)
}



#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
}





















// #[get("/<file_name>")]
// fn index(name: PathBuf) -> String {
//     let content = match fs::read_to_string(name.to_str().unwrap()) {
//         Ok(content) => content,
//         Err(_e) => format!("# error"),
//     };
//     let html : String = markdown::to_html(&content);
//     html
// }