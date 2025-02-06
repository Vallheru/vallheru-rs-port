use clap::Parser;

#[derive(Parser, Debug)]
pub struct Config {
    #[arg(
        short = 'u',
        long,
        default_value_t = String::from("postgresql://vallheru:vallheru123@localhost:5432/vallheru_db"),
    )]
    pub database_url: String,

    #[arg(long, default_value_t = 100)]
    pub database_max_connections: u32,

    #[arg(short = 'b', long, default_value_t = String::from("0.0.0.0:3004"))]
    pub bind: String,
}
