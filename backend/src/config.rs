use clap::Parser;

/// Configuration for the backend. Each field can come from a CLI flag or an env var.
#[derive(Parser, Debug)]
pub struct Config {
    /// Postgres connection URL
    #[arg(long, env)]
    pub database_url: String,

    /// Address the HTTP server listens on
    #[arg(long, env, default_value = "0.0.0.0:3000")]
    pub listen_addr: String,
}
