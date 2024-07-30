use axum::Router;
use tracing::info;

use ultimate::configuration::UltimateConfig;

pub async fn init_server(conf: &UltimateConfig, app: Router) -> ultimate::Result<()> {
    let listener = tokio::net::TcpListener::bind(conf.web().server_addr()).await.unwrap();
    let sock_addr = listener.local_addr()?;
    axum::serve(listener, app.into_make_service()).await?;
    info!("The Web Server listening on {}", sock_addr);
    Ok(())
}
