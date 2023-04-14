#[macro_use]
extern crate rocket;

use std::io;
use rocket::tokio::{
    time::{sleep, Duration},
    task::spawn_blocking,
};
use rocket::fs::FileServer;
use rocket::http::CookieJar;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/world")]
fn world() -> &'static str {
    "Other world!"
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[get("/blocking_task")]
async fn blocking_task() -> io::Result<Vec<u8>> {
    // In a real app, use rocket::fs::NamedFile or tokio::fs::File.
    let vec = spawn_blocking(|| std::fs::read("data.txt")).await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;

    Ok(vec)
}


// use std::path::{Path, PathBuf};
// use rocket::fs::NamedFile;
// #[get("/<file..>")]
// async fn files(file: PathBuf) -> Option<NamedFile> {
//     NamedFile::open(Path::new("static/").join(file)).await.ok()
// }

#[get("/foo/<_>/bar")]
fn ignore_part() -> &'static str {
    "Foo _____ bar!"
}

#[get("/ignore_everything_after/<_..>")]
fn ignore_everything() -> &'static str {
    "Hey, you're here."
}

#[get("/user/<id>")]
fn user_usize(id: usize) -> String {
    format!("User {}!", id)
}

#[get("/user/<id>", rank = 2)]
fn user_int(id: isize) -> String {
    format!("User {}!", id)
}

#[get("/user/<id>", rank = 3)]
fn user_str(id: &str) -> String {
    format!("User {}!", id)
}

#[get("/cookie_message")]
fn cookie_message(cookies: &CookieJar<'_>) -> Option<String> {
    cookies.get("message").map(|crumb| format!("Message: {}", crumb.value()))
}

#[get("/cookies")]
fn cookies(cookies: &CookieJar<'_>) -> String {
    cookies.iter().map(|crumb| format!("{}: {}", crumb.name(), crumb.value())).collect::<Vec<_>>().join(", ")
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index, world, hello, delay, blocking_task, cookie_message, cookies])
        .mount("/ignore", routes![ignore_part, ignore_everything])
        .mount("/route_ranking", routes![user_usize, user_int, user_str])
        // .mount("/static", routes![files])
        .mount("/static", FileServer::from("static/"))
        .launch()
        .await?;

    Ok(())
}
