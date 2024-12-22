#[macro_use]
extern crate rocket;
use rocket::{ fs::NamedFile, http::Status };
#[macro_use]
extern crate lazy_static;
use futures::executor::block_on;
use std::io::{ Read, Write };
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

#[get("/")]
fn index() -> NamedFile {
    // open the file in the thread and return it
    block_on(async { NamedFile::open("web/index.html").await.unwrap() })
}

#[get("/nets.html")]
fn nets() -> NamedFile {
    block_on(async { NamedFile::open("web/nets.html").await.unwrap() })
}

#[get("/uptime")]
fn uptime() -> String {
    let file = std::fs::File::open("uptime.txt").unwrap();
    let mut reader = std::io::BufReader::new(file);
    let mut time = String::new();
    reader.read_to_string(&mut time).unwrap();
    let time = time.trim().parse::<u64>().unwrap();
    let now = std::time::SystemTime
        ::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let uptime = now - time;
    let uptime = format!(
        "{} days, {} hours, {} minutes, {} seconds",
        uptime / 86400,
        (uptime % 86400) / 3600,
        (uptime % 3600) / 60,
        uptime % 60
    );
    uptime
}

#[get("/index.css")]
fn index_css() -> NamedFile {
    block_on(async { NamedFile::open("web/css/index.css").await.unwrap() })
}

#[get("/header.css")]
fn header_css() -> NamedFile {
    block_on(async { NamedFile::open("web/css/header.css").await.unwrap() })
}

#[get("/nets.css")]
fn nets_css() -> NamedFile {
    block_on(async { NamedFile::open("web/css/nets.css").await.unwrap() })
}

#[get("/preload.js")]
fn preload_js() -> NamedFile {
    block_on(async { NamedFile::open("web/js/preload.js").await.unwrap() })
}

#[get("/404.js")]
fn not_found_js() -> NamedFile {
    block_on(async { NamedFile::open("web/js/404.js").await.unwrap() })
}

#[get("/favicon.ico")]
fn favicon() -> Status {
    Status::NoContent
}

// register a 404 handler
#[catch(404)]
fn not_found() -> NamedFile {
    block_on(async { NamedFile::open("web/404.html").await.unwrap() })
}

fn init_uptime() {
    let time = std::time::SystemTime
        ::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let file = std::fs::File::create("uptime.txt").unwrap();
    let mut writer = std::io::BufWriter::new(file);
    writer.write_all(time.to_string().as_bytes()).unwrap();
}

#[launch]
fn rocket() -> _ {
    init_uptime();
    rocket::build()
        .mount("/", routes![index, nets, favicon])
        .mount("/api/", routes![uptime])
        .mount("/css", routes![index_css, header_css, nets_css])
        .mount("/js", routes![preload_js, not_found_js])
        .register("/", catchers![not_found])
}
