mod porkbun;

use axum::{
    extract::{Query, State},
    routing::get,
    Router,
};
use chrono::Local;
use clap::Parser as CliParser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use env_logger::Builder;
use serde::Deserialize;
use std::io::Write;

use crate::porkbun::Porkbun;

#[derive(CliParser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct CLI {
    /// Porkbun API key
    #[arg(long)]
    porkbun_api_key: String,

    /// Porkbun secret key
    #[arg(long)]
    porkbun_secret_key: String,

    /// Domain
    #[arg(long)]
    domain: String,

    /// Verbosity level
    #[clap(flatten)]
    verbose: Verbosity<InfoLevel>,
}

fn set_logging(cli: &CLI) {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter_module("dyndns", cli.verbose.log_level_filter())
        .init();
}

#[derive(Deserialize)]
struct Params {
    subdomain: String,
    a: Option<String>,
    aaaa: Option<String>,
    clear: Option<bool>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = CLI::parse();
    set_logging(&cli);

    let app = Router::new().route("/", get(root)).with_state(cli.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn root(State(cli): State<CLI>, params: Query<Params>) -> String {
    let (record_type, ip) = match (params.a.as_deref(), params.aaaa.as_deref()) {
        (Some(a), None) => ("A", a),
        (None, Some(aaaa)) => ("AAAA", aaaa),
        _ => return String::from("Request Error: No IP provided"),
    };

    let porkbun = Porkbun::new(cli.porkbun_api_key, cli.porkbun_secret_key, cli.domain);
    porkbun
        .delete_record(&params.subdomain, &record_type, &ip)
        .await
        .unwrap();

    if params.clear.unwrap_or(false) {
        return String::from("Record deleted");
    }

    porkbun
        .create_record(&params.subdomain, &record_type, &ip)
        .await
        .unwrap()
}
