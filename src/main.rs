#[macro_use] extern crate rocket;


use rocket::Request;
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

#[get("/")]
fn start() -> Option<Template> {

    let markdown = match fs::read_to_string("public/index") {
        Ok(markdown) => markdown,
        Err(_e) => return None,
    };

    let context = Content {
        title: format!("TODO: remove this"),
        date: format!("TODO remove this"),
        content: markdown::to_html(&markdown),
    };

    Some(Template::render("index", &context))
}

#[get("/<name..>")]
fn get_content(name: PathBuf) -> Option<Template> {
    
    let file_name : &str = name.to_str().unwrap();
    let file_name = format!("{}{}", "public/content/", file_name);

    let markdown = match fs::read_to_string(&file_name) {
        Ok(markdown) => markdown,
        Err(_e) => return None,
    };

    let seconds = fs::metadata(&file_name).unwrap()
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

    Some(Template::render("index", &context))
}

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    let context = Content {
        title: format!("TODO: Some error title"),
        date: format!("TODO: some date (turn this into option)"),
        content: req.to_string(),
    };
    Template::render("error/404", &context)
}



#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![start, get_content])
        .register("/", catchers![not_found])
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