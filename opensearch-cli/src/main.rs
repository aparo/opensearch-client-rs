use std::path::PathBuf;

use tracing::info;
use clap::{Parser, Subcommand};
use url::Url;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// Turn debugging information on
  #[clap(short, long, action = clap::ArgAction::Count)]
  verbose: u8,

  /// Opensearch server url
  #[clap(short, long, env = "OPENSEARCH_URL", default_value = "http://localhost:9200")]
  server: String,

  /// Opensearch user to be used for the connection
  #[clap(short, long, env = "OPENSEARCH_USER", default_value = "admin")]
  user: String,

  /// Opensearch password to be used for the connection
  #[clap(short, long, env = "OPENSEARCH_PASSWORD", default_value = "admin")]
  password: String,

  #[clap(subcommand)]
  command: Commands,
}
#[derive(Subcommand, Debug)]
pub enum Commands {
  /// Dump Cluster metadata
  DumpMetadata {
    /// Dump ingest pipelines
    #[clap(long, default_value = "true")]
    ingest_pipelines: bool,

    /// Dump index templates
    #[clap(long, default_value = "true")]
    index_templates: bool,

    /// Dump index components
    #[clap(long, default_value = "true")]
    index_components: bool,

    /// Sets a custom config file
    #[clap(value_name = "FILE", default_value = "output")]
    output: PathBuf,
  },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let cli = Args::parse();

  // we setup logging
  let tracing_level = match cli.verbose {
    0 => tracing::Level::WARN,
    1 => tracing::Level::INFO,
    2 => tracing::Level::DEBUG,
    _ => tracing::Level::TRACE,
  };

  tracing_subscriber::fmt()
    .compact()
    .with_thread_names(true)
    // enable everything
    .with_max_level(tracing_level)
    // sets this to be the default, global subscriber for this application.
    .init();

  let client = {
    let url = Url::parse(cli.server.clone().as_str())?;
    let mut builder = opensearch::OsClientBuilder::new().base_url(url.clone());
    builder = builder.basic_auth(url, cli.user.clone(), Some(cli.password.clone()));
    builder.build()
  };

  match &cli.command {
    Commands::DumpMetadata {
      ingest_pipelines,
      index_templates,
      index_components,
      output,
    } => {
      info!("Dumping metadata");
      info!("Output: {:?}", output);
      if *ingest_pipelines {
        info!("Ingest pipelines: {}", ingest_pipelines);
        client.tools().dump_pipelines(output.clone()).await?;
      }
      if *index_templates {
        info!("Index Templates: {}", index_templates);
        client.tools().dump_index_templates(output.clone()).await?;
      }
      if *index_components {
        info!("Index Components: {}", index_components);
        client.tools().dump_index_components(output.clone()).await?;
      }
    }
  }

  Ok(())
}
