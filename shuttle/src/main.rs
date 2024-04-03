use shuttle_runtime::tokio::time;
use shuttle_runtime::SecretStore;
use std::process::Command;

#[shuttle_runtime::main]
#[allow(unused_must_use)]
async fn shuttle_main(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> Result<MyService, shuttle_runtime::Error> {

    // TODO: add necessary commands here
    // npm install
    Command::new("npm")
        .arg("install")
        .spawn()
        .expect("Failed to install npm dependencies")
        .wait();

    // npm start
    Command::new("npm")
        .arg("start")
        // environment variables from Secrets.toml
        .envs(secrets.clone().into_iter())
        .spawn()
        .expect("Failed to start Node.js process")
        .wait();

    Ok(MyService {})
}

struct MyService {}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for MyService {
    async fn bind(self, _addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        time::sleep(time::Duration::from_secs(1)).await;
        Ok(())
    }
}
