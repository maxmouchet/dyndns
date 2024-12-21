mod porkbun;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
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

    // Subdomain
    #[arg(long)]
    subdomain: Option<String>,

    /// Verbosity level
    #[clap(flatten)]
    verbose: Verbosity<InfoLevel>,

    /// Authentication token
    #[arg(long)]
    token: String,
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
    token: String,
    subdomain: String,
    a: Option<String>,
    aaaa: Option<String>,
    clear: Option<bool>,
}

use axum::response::Response as AxumResponse;

#[derive(serde::Serialize)]
struct Response {
    message: String,
    domain: String,
}

impl IntoResponse for Response {
    fn into_response(self) -> AxumResponse {
        AxumResponse::new(axum::body::Body::empty())
    }
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

fn response(status: StatusCode, message: &str, domain: &str) -> (StatusCode, Json<Response>) {
    (
        status,
        Json(Response {
            message: String::from(message),
            domain: String::from(domain),
        }),
    )
}

#[axum::debug_handler]
async fn root(State(cli): State<CLI>, params: Query<Params>) -> impl IntoResponse {
    if cli.token != params.token {
        return response(StatusCode::UNAUTHORIZED, "Unauthorized: Invalid token", "");
    }

    let (record_type, ip) = match (params.a.as_deref(), params.aaaa.as_deref()) {
        (Some(a), None) => ("A", a),
        (None, Some(aaaa)) => ("AAAA", aaaa),
        _ => {
            return response(
                StatusCode::BAD_REQUEST,
                "Bad request: Either A or AAAA record must be provided",
                "",
            )
        }
    };

    let mut subdomain = params.subdomain.clone();
    if !cli.subdomain.is_none() {
        subdomain = format!("{}.{}", params.subdomain, cli.subdomain.unwrap());
    }
    let domain = format!("{}.{}", subdomain, cli.domain);

    let porkbun = Porkbun::new(cli.porkbun_api_key, cli.porkbun_secret_key, cli.domain);
    porkbun
        .delete_record(&subdomain, &record_type, &ip)
        .await
        .unwrap();

    if params.clear.unwrap_or(false) {
        return response(StatusCode::OK, "OK", &domain);
    }

    porkbun
        .create_record(&subdomain, &record_type, &ip)
        .await
        .unwrap();

    response(StatusCode::OK, "OK", &domain)
}
