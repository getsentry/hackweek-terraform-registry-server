use poem::{
    listener::{Listener, RustlsCertificate, RustlsConfig, TcpListener},
    Server,
};
use tokio::fs;

use terraform_registry_server::{build_app, configuration::Settings};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug")
    }
    tracing_subscriber::fmt::init();

    let settings = Settings::default();

    tracing::info!(
        "starting HTTP server at {}:{}",
        &settings.host,
        &settings.port
    );

    let app = build_app(&settings);

    if settings.tls.enabled {
        let key = fs::read(&settings.tls.key).await?;
        let cert = fs::read(&settings.tls.cert).await?;
        let listener = TcpListener::bind((settings.host, settings.port))
            .rustls(RustlsConfig::new().fallback(RustlsCertificate::new().key(key).cert(cert)));
        Server::new(listener).run(app).await
    } else {
        let listener = TcpListener::bind((settings.host, settings.port));
        Server::new(listener).run(app).await
    }
}
