use clap::Parser;


/// Configuration for env variables
#[derive(Parser, Debug)]
pub struct Config {

    /// The connection URL for the database
    #[arg(long, env)]
    pub database_url: String,
}