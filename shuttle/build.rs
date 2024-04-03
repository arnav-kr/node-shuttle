#[allow(unused_must_use)]
fn main() {
    println!("Installing Dependencies...");
    // Install external dependency (in the shuttle container only)
    if std::env::var("SHUTTLE").is_ok() {
        std::process::Command::new("apt")
            .arg("install")
            .arg("-y")
            .arg("nodejs")
            .arg("npm")
            // TODO: Add More Dependencies Here
            // .arg("yarn")
            .spawn()
            .expect("Failed to install dependencies")
            .wait();
    }
}
