use std::fmt;
use std::net::SocketAddr;

use clap::Parser;

/// Configuration for the backend. Each field can come from a CLI flag or an env var.
#[derive(Parser)]
pub struct Config {
    /// Postgres connection URL
    #[arg(long, env)]
    pub database_url: String,

    /// Address the HTTP server listens on
    #[arg(long, env, default_value = "0.0.0.0:3000")]
    pub listen_addr: SocketAddr,
}

// Manual Debug so the database password never ends up in logs.
impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config")
            .field("database_url", &"<redacted>")
            .field("listen_addr", &self.listen_addr)
            .finish()
    }
}
