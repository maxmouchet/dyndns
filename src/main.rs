mod porkbun;

use chrono::Local;
use clap::Parser as CliParser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use env_logger::Builder;
use std::io::Write;

use crate::porkbun::Porkbun;

#[derive(CliParser, Debug)]
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = CLI::parse();
    set_logging(&cli);

    let porkbun = Porkbun::new(cli.porkbun_api_key, cli.porkbun_secret_key, cli.domain);
    let txt = porkbun.get_record("ams.sw.infra", "A").await?;
    println!("{}", txt);

    Ok(())
}
