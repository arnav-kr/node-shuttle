# Node.js on shuttle.rs Template

## Pre-requisites
- [rust](https://www.rust-lang.org/tools/install) installed
- [cargo-shuttle CLI](https://docs.shuttle.rs/getting-started/installation) installed
- [Node.js](https://nodejs.org/en/download/) installed (obv)

## Getting Started
- Create a new project using the template
  ```bash
  cargo shuttle init --from arnav-kr/node-shuttle
  ```
- rename `shuttle/Secrets.example.toml` to `shuttle/Secrets.toml` and put your environment variables there. Those will be deployed to the shuttle.rs server.

- restart the project
  ```bash
  npm run shuttle:restart
  ```
- deploy the project
  ```bash
  npm run shuttle:deploy
  ```

> [!NOTE]
> First Deploy might take some time to finish (maybe a lot) but subsequent deploys will be faster.

> [!NOTE]
> `package.json` contains the `shuttle:` scripts to interact with the shuttle.rs server. You can use them to login, deploy, start, restart, and stop the server.


## Deploy via GitHub Actions
Cosidering the deploy limit per day, the deploy workflow is opt-in by default i.e. you need to manually trigger it by putting `[deploy]` anywhere in your commit message.

> [!IMPORTANT]
> To deploy using Github Actions, you need to specify the [`SHUTTLE_TOKEN`](https://console.shuttle.rs/login) in the repository secrets along with any other environment variable you are going to use in your project.\
> You can add or remove variables in [`.github/workflows/deploy.yml`](.github/workflows/deploy.yml) file.\
> This template requires the following environment variables to be set in the repository secrets:\
> - `SHUTTLE_TOKEN`: The token to authenticate with the shuttle.rs server.\
> - `NODE_ENV`: The project id of the shuttle.rs project.\
> - `DISCORD_TOKEN`: The discord bot token.\
> Not specifying them in the repository secrets will result in a failed deployment.

> [!TIP]
> To save your build hours, you may run the first deploy manually and then use the GitHub Actions to deploy the later changes.


## Development
use general node workflow for development.

## License
This project is licensed under the AGPL 3.0 License - see the [LICENSE](LICENSE) file for details

## Author
[Arnav Kumar (@arnav-kr)](https://github.com/arnav-kr)