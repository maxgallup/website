#[macro_use] extern crate rocket;



use rocket::Request;
use rocket_dyn_templates::{Template};
use std::{time::SystemTime};
use std::fs;

use serde::{Serialize};

use chrono::naive::NaiveDateTime;


#[derive(Serialize)]
struct Content {
    title: String,
    date: String,
    content: Option<String>,
}

#[derive(Serialize)]
struct Item {
    title: String,
    date: String
}

#[derive(Serialize)]
struct ContentDir {
    title: String,
    items: Vec<Item>
}

fn get_date(path: &str) -> String {
    let seconds = fs::metadata(path).unwrap()
    .modified().unwrap_or(SystemTime::now())
    .duration_since(SystemTime::UNIX_EPOCH).unwrap()
    .as_secs().try_into().unwrap();

    NaiveDateTime::from_timestamp(seconds, 0)
    .format("%Y-%m-%d").to_string()
}


// --- START PAGE --- 
#[get("/")]
fn start_page() -> Option<Template> {

    let markdown = match fs::read_to_string("public/start") {
        Ok(markdown) => markdown,
        Err(_e) => return None,
    };

    let context = Content {
        title: format!("TODO: remove this"),
        date: format!("TODO remove this"),
        content: Some(markdown::to_html(&markdown)),
    };

    Some(Template::render("content", &context))
}

// --- CONTENT DIRECTORIES ---
#[get("/<dir>")]
fn get_content_dir(dir: String) -> Option<Template> {

    let path = format!("public/content/{}", dir);

    let mut base = ContentDir {
        title: dir,
        items: vec![],
    };

    let entries = match fs::read_dir(path) {
        Ok(x) => x,
        Err(_e) => return None,
    };

    for entry in entries {
        if let Ok(entry) = entry {

            let item = Item {
                title: entry.file_name().to_str().unwrap().to_string(),
                date: get_date(entry.path().to_str().unwrap()),
            };
            
            base.items.push(item);
        }
    };

    Some(Template::render("content-dir", &base))
}

#[get("/<dir>/<name>")]
fn get_content(dir: String, name: String) -> Option<Template> {

    let path = format!("public/content/{}/{}", dir, name);

    let markdown = match fs::read_to_string(&path) {
        Ok(markdown) => markdown,
        Err(_e) => return None,
    };

    let context = Content {
        title: format!("TODO: Some title"),
        date: get_date(&path),
        content: Some(markdown::to_html(&markdown)),
    };

    Some(Template::render("content", &context))
}

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    let context = Content {
        title: format!("TODO: Some error title"),
        date: format!("TODO: some date (turn this into option)"),
        content: Some(req.to_string()),
    };
    Template::render("error/404", &context)
}



#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![start_page, get_content, get_content_dir])
        .register("/", catchers![not_found])
        .attach(Template::fairing())
}




















