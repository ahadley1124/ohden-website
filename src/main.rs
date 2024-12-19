#[macro_use]
extern crate rocket;
use rocket::fs::NamedFile;
use futures::executor::block_on;
use std::io::{ Read, Write };

#[get("/")]
fn index() -> NamedFile {
    // open the file in the thread and return it
    block_on(async { NamedFile::open("web/index.html").await.unwrap() })
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
    rocket
        ::build()
        .mount("/", routes![index])
        .mount("/api/", routes![uptime])
        .mount("/css", routes![index_css])
}
