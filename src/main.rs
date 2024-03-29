#[macro_use]
extern crate rocket;

use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::SystemTime;

use rocket::fs::NamedFile;
use rocket::Request;
use rocket_dyn_templates::Template;

use serde::Serialize;

use chrono::naive::NaiveDateTime;

#[derive(Serialize)]
struct StartPage {
    title: Option<String>,
    intro: Option<String>,
    cards: Vec<ContentDir>,
}

#[derive(Serialize)]
struct Content {
    title: Option<String>,
    date: Option<String>,
    content: Option<String>,
}

#[derive(Serialize)]
struct Item {
    title: String,
    link: String,
    date: String,
}

#[derive(Serialize)]
struct ContentDir {
    title: String,
    items: Vec<Item>,
}

fn get_date(path: &str) -> String {
    let seconds = fs::metadata(path)
        .unwrap()
        .modified()
        .unwrap_or(SystemTime::now())
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .try_into()
        .unwrap();

    NaiveDateTime::from_timestamp(seconds, 0)
        .format("%d-%m-%Y")
        .to_string()
}

fn get_title(path: &str) -> Option<String> {
    let file = match fs::File::open(path) {
        Ok(file) => file,
        Err(_) => return None,
    };

    let mut buffer = BufReader::new(file);

    let mut first_line = String::new();
    buffer
        .read_line(&mut first_line)
        .expect("unable to read line");

    let last_hash = first_line
        .char_indices()
        .skip_while(|&(_, c)| c == '#')
        .next()
        .map_or(0, |(idx, _)| idx);

    Some(first_line[last_hash..].trim().into())
}

fn generate_content_dir(path: &str) -> Option<ContentDir> {
    let full_path = format!("public/content/{}", path);

    let mut base = ContentDir {
        title: path.to_string(),
        items: vec![],
    };

    let entries = match fs::read_dir(full_path) {
        Ok(x) => x,
        Err(_e) => return None,
    };

    for entry in entries {
        if let Ok(entry) = entry {
            let title = match get_title(entry.path().to_str().unwrap()) {
                Some(s) => s,
                None => return None,
            };

            let item = Item {
                title: title,
                link: entry.file_name().to_str().unwrap().to_string(),
                date: get_date(entry.path().to_str().unwrap()),
            };

            base.items.push(item);
        }
    }
    Some(base)
}

// --- START PAGE ---
#[get("/")]
fn start_page() -> Option<Template> {
    let markdown = match fs::read_to_string("public/start") {
        Ok(markdown) => markdown,
        Err(_e) => return None,
    };

    let mut cards_vec: Vec<ContentDir> = vec![];

    let entries = match fs::read_dir("public/content") {
        Ok(x) => x,
        Err(_e) => return None,
    };

    for entry in entries {
        if let Ok(entry) = entry {
            if entry.path().is_dir() {
                let temp = match generate_content_dir(entry.file_name().to_str().unwrap()) {
                    Some(x) => x,
                    None => return None,
                };
                cards_vec.push(temp);
            }
        }
    }

    let context = StartPage {
        title: Some("@maxgallup".to_string()),
        intro: Some(markdown::to_html(&markdown)),
        cards: cards_vec,
    };

    Some(Template::render("start", &context))
}

fn generate_home_content(name: &String) -> Option<Template> {
    let path = format!("public/{}", name);

    let markdown = match fs::read_to_string(&path) {
        Ok(markdown) => markdown,
        Err(_e) => return None,
    };

    let context = Content {
        title: Some(format!("{}", name)),
        date: Some(get_date(&path)),
        content: Some(markdown::to_html(&markdown)),
    };

    Some(Template::render("content", &context))
}

// --- MEDIA ---
#[get("/media/<name>")]
async fn media(name: &str) -> Option<NamedFile> {
    let path = format!("public/media/{}", name);
    NamedFile::open(Path::new(&path)).await.ok()
}

// --- CONTENT DIRECTORIES ---
#[get("/<dir>")]
fn get_content_dir(dir: String) -> Option<Template> {
    let base = match generate_content_dir(&dir) {
        Some(x) => x,
        None => return generate_home_content(&dir),
    };

    Some(Template::render("content-dir", &base))
}

// --- CONTENT ---
#[get("/<dir>/<name>")]
fn get_content(dir: String, name: String) -> Option<Template> {
    let path = format!("public/content/{}/{}", dir, name);

    let markdown = match fs::read_to_string(&path) {
        Ok(markdown) => markdown,
        Err(_e) => return None,
    };

    let context = Content {
        title: Some(format!("{}/{}", dir, name)),
        date: Some(get_date(&path)),
        content: Some(markdown::to_html(&markdown)),
    };

    Some(Template::render("content", &context))
}

// --- FONTS ---
#[get("/ProximaNovaThin.otf")]
async fn font1() -> Option<NamedFile> {
    NamedFile::open(Path::new("public/fonts/ProximaNovaThin.otf"))
        .await
        .ok()
}

#[get("/ProximaNovaRegular.otf")]
async fn font2() -> Option<NamedFile> {
    NamedFile::open(Path::new("public/fonts/ProximaNovaRegular.otf"))
        .await
        .ok()
}

// --- CV PAGE ---
#[get("/cv")]
async fn cv_page() -> Option<NamedFile> {
    NamedFile::open(Path::new("public/cv.pdf")).await.ok()
}

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    let context = Content {
        title: Some("404 - Not Found".to_string()),
        date: None,
        content: Some(req.uri().to_string()),
    };

    Template::render("404", &context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                start_page,
                cv_page,
                get_content,
                get_content_dir,
                font1,
                font2,
                media,
            ],
        )
        .mount("/start", routes![start_page])
        .register("/", catchers![not_found])
        .attach(Template::fairing())
}
