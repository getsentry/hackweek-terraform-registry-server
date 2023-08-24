use poem::{listener::TcpListener, Server};

use terraform_registry_server::{build_app, configuration::Settings};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let settings = Settings::default();

    log::info!(
        "starting HTTP server at {}:{}",
        &settings.host,
        &settings.port
    );

    let app = build_app(&settings);

    Server::new(TcpListener::bind((settings.host, settings.port)))
        .run(app)
        .await
}
