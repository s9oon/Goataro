use backend::{config::Config, routes, state::AppState};
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse();

    if !config.engine_path.is_file() {
        return Err(format!("engine not found: {}", config.engine_path.display()).into());
    }

    let state = AppState::new(&config.database_url).await?;
    let app = routes::router(state);

    let listener = tokio::net::TcpListener::bind(config.listen_addr).await?;
    println!("listening on {}", config.listen_addr);
    axum::serve(listener, app).await?;

    Ok(())
}
