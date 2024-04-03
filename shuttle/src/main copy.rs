use std::env;
use std::path::Path;
use std::process::Command;
use shuttle_runtime::SecretStore;
use rocket::{get, routes};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secrets: SecretStore) -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().mount("/", routes![index]);
    
    env::set_current_dir(Path::new("./nodejs")).expect("Failed to Change Working Directory");
    
    Command::new("pwd")
        .arg("-v")
        .spawn()
        .expect("Failed to Change Working Directory")
        .wait();
    Command::new("npm")
        .arg("install")
        .spawn()
        .expect("failed to install npm dependencies")
        .wait();
    Command::new("node")
        .arg("./src/index.js")
        .envs(secrets.into_iter())
        .spawn()
        .expect("Failed to start Node.js process")
        .wait();
    Ok(rocket.into())
}
