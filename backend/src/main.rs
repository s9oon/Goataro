use clap::Parser;
use sqlx::{
    postgres::PgPoolOptions,
    postgres::PgPool,
};
use axum::{ 
    extract::State,
    http::StatusCode,
    routing::get, 
    Router, 
};

#[derive(Clone)]
struct AppState {
    pub pool: PgPool,
}

/// Configuration for backend env
#[derive(Parser, Debug)]
struct Config {

    /// The URL for the database
    #[arg(long = "database_url")]
    database_url: String,
}

async fn health(State(state): State<AppState>) -> Result<(), StatusCode> {

    // Database Query
    match sqlx::query("SELECT 1").execute(&state.pool).await {
        Ok(_) => Ok(()),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    //idk what to say tbh
    let config = Config::parse();
    println!("{}", config.database_url);

    // App Dependcies 
    let state = AppState {
        // Database Connection
        pool: PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url).await?
    };

    // build our application with a single route
    let app = Router::new().route("/", get(health)).with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}