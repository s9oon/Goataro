mod config;
mod routes;
mod state;

use clap::Parser;

use config::Config;
use state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse();

    let state = AppState::new(&config.database_url).await?;
    let app = routes::router(state);

    let listener = tokio::net::TcpListener::bind(&config.listen_addr).await?;
    println!("listening on {}", config.listen_addr);
    axum::serve(listener, app).await?;

    Ok(())
}
