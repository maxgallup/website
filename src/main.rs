#[macro_use] extern crate rocket;
extern crate markdown;


use std::path::PathBuf;
use std::fs;


#[get("/<file..>")]
fn index(file: PathBuf) -> String {

    println!("{:?}", file.to_str());

    let content = fs::read_to_string(file.as_path())
                    .expect("__unable to open file__");

    let html : String = markdown::to_html(&content);
    html
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
