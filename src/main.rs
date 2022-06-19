use std::net::SocketAddr;
use std::time::Duration;

use anyhow::Result;
use clap::Parser;
use env_logger::Env;
use log::error;
use prometheus_exporter::prometheus::register_gauge;
use tokio::time::sleep;

use crate::repository_stats::StatsFetcher;

mod github_client;
mod repository_stats;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Repository to monitor
    #[clap(short, long, value_parser)]
    repository: String,

    /// Port to listen on
    #[clap(short, long, default_value_t = 8000, value_parser)]
    port: u16,

    /// Timeout for GitHub requests in seconds
    #[clap(short, long, default_value_t = 5, value_parser)]
    timeout: u16,

    /// Connect timeout for GitHub requests in seconds
    #[clap(short, long, default_value_t = 5, value_parser)]
    connect_timeout: u16,

    /// Interval in seconds to refresh GitHub stats
    #[clap(short, long, default_value_t = 60, value_parser)]
    fetch_interval: u16,

    /// Change the default log level
    #[clap(long, default_value_t = String::from("info"), value_parser, possible_values = &["info", "warn", "error", "debug"])]
    log_level: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    env_logger::Builder::from_env(Env::default().default_filter_or(args.log_level)).init();

    let issue_count_metric =
        register_gauge!("open_issues_count", "count of issues in the repository")
            .expect("Could not register 'open_issues_count' metric");

    let stargazers_count_metric =
        register_gauge!("stargazers_count", "count of stars in the repository")
            .expect("Could not register 'stargazers_count' metric");

    let addr: SocketAddr = format!("0.0.0.0:{}", args.port)
        .parse()
        .expect("failed to parse binding");
    let exporter = prometheus_exporter::start(addr).expect("failed to start prometheus exporter");

    let github_client = github_client::build(args.connect_timeout.into(), args.timeout.into())
        .expect("failed to build github client");
    let stats_fetcher = StatsFetcher::new(args.repository, github_client);

    tokio::spawn(async move {
        loop {
            match stats_fetcher.fetch().await {
                Ok(stats) => {
                    issue_count_metric.set(stats.open_issues_count as f64);
                    stargazers_count_metric.set(stats.stargazers_count as f64);
                }
                Err(err) => {
                    error!("failed to fetch stats: {}", err);
                }
            }

            sleep(Duration::from_secs(args.fetch_interval.into())).await;
        }
    });

    loop {
        let _guard = exporter.wait_request();
    }
}
