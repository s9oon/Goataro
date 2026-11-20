use backend::{config::Config, routes, state::AppState, engine::process::EngineProcess};
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse();

    let state = AppState::new(&config.database_url).await?;
    let app = routes::router(state);

    let mut engine_process = EngineProcess::spawn(std::path::Path::new("stockfish"))?;

    EngineProcess::ping(&mut engine_process).await?;

    let listener = tokio::net::TcpListener::bind(config.listen_addr).await?;
    println!("listening on {}", config.listen_addr);
    axum::serve(listener, app).await?;

    Ok(())
}
